use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Range;

use crate::assembler::{Assembler, Error, ErrorLevel, TokenReader};
use crate::assembler::directive_impl::Directives;
use crate::assembler::error_impl::ErrorType;
use crate::assembler::expression_impl::ExpressionParser;
use crate::assembler::instruction_encoder::InstructionEncoder;
use crate::assembler::tokens::{AluOp, OpCode, Token};
use crate::assembler::tokens::Op::Equals;
use crate::assembler::tokens::RotOp::{Rl, Rlc, Rr, Rrc, Sla, Sll, Sra, Srl};
use crate::assembler::tokens::Token::{ConstLabel, Number, Operator, AddressIndirect, ConstLabelIndirect};
use crate::assembler::error_impl::ErrorType::SyntaxError;
use crate::assembler::bank::Bank;
use crate::assembler::reg_pair::HighLow;

impl Assembler {
    pub fn new() -> Assembler {
        let context = Default::default();
        Assembler {
            context,
            tokens: vec![],
            origin: 0,
            bank: Bank::new(),
            console_output: false,
            total_lines: 0,
            expr: ExpressionParser::new(),
            z80n_enabled: false,
            cspect_enabled: false,
        }
    }

    pub fn enable_z80n(&mut self, enabled: bool) -> &mut Assembler {
        self.z80n_enabled = enabled;
        self
    }

    pub fn enable_console(&mut self, enabled: bool) -> &mut Assembler {
        self.console_output = enabled;
        self
    }

    pub fn enable_cspect(&mut self, enabled: bool) -> &mut Assembler {
        self.cspect_enabled = enabled;
        self
    }

    pub fn assemble(&mut self, file_name: &str) -> Result<(), Error> {
        if self.console_output {
            dark_green_ln!("First pass...");
        }
        self.first_pass(file_name)?;
        if self.console_output {
            dark_green_ln!("Second pass...");
        }
        self.second_pass()
    }


    pub(crate) fn first_pass(&mut self, file_name: &str) -> Result<(), Error> {
//        self.context.line_number.push(0);
//        self.context.file_name.push(file_name.to_string());
        self.context.enter(file_name);
        let file = File::open(file_name)?;
        let buf = BufReader::new(file);
        let mut reader = TokenReader::new(buf);
        reader.delimiters(",").operators("()*/+-<>=^&|");
        self.tokens.clear();
        loop {
            let tokens = &mut reader.read_line()?;
            self.total_lines += 1;
            match tokens.first() {
                Some(Token::EndOfFile) => break,
                _ => self.translate(tokens)?
            }
        }
        self.context.leave();
        Ok(())
    }

    pub fn second_pass(&mut self) -> Result<(), Error> {
        while let Some(mut fwd_ref) = self.context.next_forward_ref() {
            let mut data: isize;
            if fwd_ref.is_expression {
                data = match self.expr.eval(&mut self.context, fwd_ref.expression.as_mut()) {
                    Ok(n) => n,
                    Err(e) => return Err(self.error_second_pass(e, fwd_ref.line_no, &fwd_ref.file_name)),
                }
            } else {
                data = match self.context.get_label_or_constant_value(fwd_ref.label.as_str()) {
                    Ok(n) => n,
                    Err(_) => return Err(self.error_second_pass(SyntaxError, fwd_ref.line_no, &fwd_ref.file_name)),
                }
            }
            let index = fwd_ref.pc as usize - self.origin as usize;
            if fwd_ref.is_relative {
                let offset = data - (fwd_ref.pc + 1) as isize;
                self.bank[index] = offset as u8;
            } else {
                for d in 0..fwd_ref.byte_count as usize {
                    self.bank[index + d] = (data & 0xff) as u8;
                    data = data >> 8;
                }
                // fixup the z80n "push nnnn" endiannnessssss
                if self.z80n_enabled && fwd_ref.byte_count == 2 && index > 1 && self.bank[index - 2] == 0xed && self.bank[index - 1] == 0x8a {
                    let b = self.bank[index + 1];
                    self.bank[index + 1] = self.bank[index];
                    self.bank[index] = b;
                }
            }
        }
        Ok(())
    }

    pub fn save_raw(&mut self, file_name: &str) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        file.write_all(self.bank.as_slice())?;
        Ok(())
    }

    pub fn warn(&mut self, t: ErrorType) {
        if self.console_output {
            cyan_ln!("[{} : {}] Warning: {}", self.context.current_file_name(), self.context.current_line_number(), t.to_string());
        }
    }

    pub fn info(&mut self, m: &str) {
        if self.console_output {
            yellow_ln!("[{} : {}] {}",  self.context.current_file_name(), self.context.current_line_number(), m);
        }
    }

    pub fn error_second_pass(&mut self, t: ErrorType, line_no: isize, file_name: &str) -> Error {
        Error {
            line_no,
            message: t.to_string(),
            level: ErrorLevel::Fatal,
            file_name: file_name.to_string(),
        }
    }

    pub fn relative(&mut self) -> Result<u8, Error> {
        match self.take_token()? {
            Number(n) => Ok((n - (self.context.offset_pc(2))) as u8),
            ConstLabel(s) => {
                let mut addr = self.context.try_resolve_label(&s, 1, true) as isize;
                let pc = (self.context.offset_pc(2)) as isize;
                if addr == 0 {
                    addr = pc;
                }
                Ok(((addr - pc) as u16) as u8)
            }
            _ => Err(self.context.error(ErrorType::SyntaxError))
        }
    }

    pub(crate) fn expect_byte(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..256, 1, ErrorType::ByteTrunctated, instr_size)
    }

    pub(crate) fn expect_word(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..65536, 2, ErrorType::WordTruncated, instr_size)
    }

    fn expect_number_in_range(&mut self, range: Range<isize>, count: isize, error_type: ErrorType, instr_size: isize) -> Result<isize, Error> {
        match self.expr.parse(&mut self.context, &mut self.tokens, instr_size, count) {
            Ok(Some(n)) => {
                if !range.contains(&n) {
                    self.warn(error_type);
                }
                Ok(n)
            }
            Ok(None) => return Err(self.context.error(ErrorType::SyntaxError)),
            Err(e) => return Err(self.context.error(e))
        }
    }

    pub(crate) fn decode_number(&mut self, token: &Token, pc_offset: isize) -> Result<Option<isize>, Error> {
        match &token {
            Number(n) => Ok(Some(*n)),
            AddressIndirect(a) => Ok(Some(*a as isize)),
            ConstLabelIndirect(l) => Ok(Some(self.context.try_resolve_label(l, pc_offset, false) as isize)),
            ConstLabel(l) => if let Some(n) = self.context.get_constant(l) {
                Ok(Some(n))
            } else {
                Err(self.context.error(ErrorType::BadConstant))
            }
            _ => Ok(None)
        }
    }

    pub fn take_token(&mut self) -> Result<Token, Error> {
        if let Some(tok) = self.tokens.pop() {
            return Ok(tok);
        }
        Err(self.context.error(ErrorType::UnexpectedEndOfLine))
    }

    pub fn next_token_is(&mut self, tok: &Token) -> bool {
        if let Some(t) = self.tokens.last() {
            return t == tok;
        }
        return false;
    }

    pub fn expect_token(&mut self, tok: Token) -> Result<(), Error> {
        let t = self.take_token()?;
        if t != tok {
            return Err(self.context.error(ErrorType::SyntaxError));
        }
        Ok(())
    }

    pub(crate) fn emit(&mut self, b: &[u8]) -> Result<(), Error> {
        let pc = self.context.offset_pc(b.len() as isize);
        if pc > 65535 {
            self.warn(ErrorType::PCOverflow)
        }
        self.context.pc(pc);
        self.bank.append(&mut b.to_vec());
        Ok(())
    }

    pub(crate) fn emit_byte(&mut self, b: u8) -> Result<(), Error> {
        let pc = self.context.offset_pc(1);
        if pc > 65535 {
            self.warn(ErrorType::PCOverflow)
        }
        self.context.pc(pc);
        self.bank.push(b);
        Ok(())
    }

    pub(crate) fn emit_word(&mut self, word: isize) -> Result<(), Error> {
        if word < 0 || word > 65535 {
            self.warn(ErrorType::WordTruncated);
        }
        let w = word as u16;
        let pc = self.context.offset_pc(2);
        if pc > 65535 {
            self.warn(ErrorType::PCOverflow)
        }
        self.context.pc(pc);
        self.bank.push(w.lo());
        self.bank.push(w.hi());
        Ok(())
    }

    fn handle_opcodes(&mut self, op: OpCode) -> Result<(), Error> {
        return match op {
            OpCode::Nop => self.emit_byte(0),
            OpCode::Adc => self.alu_op_r(AluOp::Adc, 1, 0),
            OpCode::Add => self.alu_op_r(AluOp::Add, 0, 1),
            OpCode::And => self.alu_op(AluOp::And),
            OpCode::Bit => self.bit_res_set(1),
            OpCode::Call => self.call_jp(1, 5),
            OpCode::Ccf => self.emit_byte(0x3F),
            OpCode::Cp => self.alu_op(AluOp::Cp),
            OpCode::Cpd => self.emit(&[0xED, 0xA9]),
            OpCode::Cpdr => self.emit(&[0xED, 0xB9]),
            OpCode::Cpi => self.emit(&[0xED, 0xA1]),
            OpCode::Cpir => self.emit(&[0xED, 0xB1]),
            OpCode::Cpl => self.emit_byte(0x2F),
            OpCode::Daa => self.emit_byte(0x27),
            OpCode::Dec => self.inc_dec(1),
            OpCode::Di => self.emit_byte(0xF3),
            OpCode::Djnz => self.jr(true),
            OpCode::Ei => self.emit_byte(0xFB),
            OpCode::Ex => self.ex(),
            OpCode::Exx => self.emit_byte(0xD9),
            OpCode::Halt => self.emit_byte(0x76),
            OpCode::Im => self.im(),
            OpCode::In => self.io_op(3),
            OpCode::Inc => self.inc_dec(0),
            OpCode::Ind => self.emit(&[0xED, 0xAA]),
            OpCode::Indr => self.emit(&[0xED, 0xBA]),
            OpCode::Ini => self.emit(&[0xED, 0xA2]),
            OpCode::Inir => self.emit(&[0xED, 0xB2]),
            OpCode::Jr => self.jr(false),
            OpCode::Jp => self.jp(),
            OpCode::Ld => self.load(),
            OpCode::Ldd => self.emit(&[0xED, 0xA8]),
            OpCode::Lddr => self.emit(&[0xED, 0xB8]),
            OpCode::Ldi => self.emit(&[0xED, 0xA0]),
            OpCode::Ldir => self.emit(&[0xED, 0xB0]),
            OpCode::Neg => self.emit(&[0xED, 0x44]),
            OpCode::Or => self.alu_op(AluOp::Or),
            OpCode::Otdr => self.emit(&[0xED, 0xBB]),
            OpCode::Otir => self.emit(&[0xED, 0xB3]),
            OpCode::Out => self.io_op(2),
            OpCode::Outd => self.emit(&[0xED, 0xAB]),
            OpCode::Outi => self.emit(&[0xED, 0xA3]),
            OpCode::Pop => self.push_pop(1),
            OpCode::Push => self.push_pop(5),
            OpCode::Res => self.bit_res_set(2),
            OpCode::Ret => self.ret(),
            OpCode::Reti => self.emit(&[0xED, 0x4D]),
            OpCode::Retn => self.emit(&[0xED, 0x45]),
            OpCode::Rl => self.rot(Rl),
            OpCode::Rla => self.emit_byte(0x17),
            OpCode::Rlc => self.rot(Rlc),
            OpCode::Rlca => self.emit_byte(0x07),
            OpCode::Rld => self.emit(&[0xED, 0x6F]),
            OpCode::Rr => self.rot(Rr),
            OpCode::Rra => self.emit_byte(0x1F),
            OpCode::Rrc => self.rot(Rrc),
            OpCode::Rrca => self.emit_byte(0x0F),
            OpCode::Rrd => self.emit(&[0xED, 0x67]),
            OpCode::Rst => self.rst(),
            OpCode::Sbc => self.alu_op_r(AluOp::Sbc, 1, 1),
            OpCode::Scf => self.emit_byte(0x37),
            OpCode::Set => self.bit_res_set(3),
            OpCode::Sla => self.rot(Sla),
            OpCode::Sll => self.rot(Sll),
            OpCode::Sra => self.rot(Sra),
            OpCode::Srl => self.rot(Srl),
            OpCode::Sub => self.alu_op(AluOp::Sub),
            OpCode::Xor => self.alu_op(AluOp::Xor),
            _ => {
                self.encode_z80n(&op)?;
                self.encode_cspect(&op)?;
                return Ok(());
            }
        };
    }

    fn encode_cspect(&mut self, op: &OpCode) -> Result<(), Error> {
        let code = match op {
            OpCode::Break => Some(vec![0xDD, 1]),
            OpCode::Exit => Some(vec![0xDD, 0]),
            _ => None
        };
        if let Some(b) = code {
            if !self.cspect_enabled {
                return Err(self.context.error(ErrorType::CSpectDisabled));
            }
            self.emit(&b)?;
        }
        Ok(())
    }

    fn encode_z80n(&mut self, op: &OpCode) -> Result<(), Error> {
        let code = match op {
            OpCode::Ldix => Some(vec![0xED, 0xA4]),
            OpCode::Ldws => Some(vec![0xED, 0xA5]),
            OpCode::Ldirx => Some(vec![0xED, 0xB4]),
            OpCode::Lddx => Some(vec![0xED, 0xAC]),
            OpCode::Lddrx => Some(vec![0xED, 0xBC]),
            OpCode::Ldpirx => Some(vec![0xED, 0xB7]),
            OpCode::Outinb => Some(vec![0xED, 0x90]),
            OpCode::Mul => return self.mul(),
            OpCode::Swapnib => Some(vec![0xED, 0x23]),
            OpCode::Mirror => Some(vec![0xED, 0x24]),
            OpCode::Nextreg => return self.next_reg(),
            OpCode::Pixeldn => Some(vec![0xED, 0x93]),
            OpCode::Pixelad => Some(vec![0xED, 0x94]),
            OpCode::Setae => Some(vec![0xED, 0x95]),
            OpCode::Test => Some(vec![0xED, 0x27, self.expect_byte(2)? as u8]),
            _ => None
        };
        if let Some(b) = code {
            if !self.z80n_enabled {
                return Err(self.context.error(ErrorType::Z80NDisabled));
            }
            self.emit(&b)?
        }
        Ok(())
    }

    fn handle_label(&mut self, l: &str) -> Result<(), Error> {
        if self.next_token_is(&Operator(Equals)) {
            self.tokens.pop();
            match self.expr.parse(&mut self.context, &mut self.tokens, 0, -1) {
                Ok(Some(n)) => self.context.add_constant(l.to_string(), n)?,
                Ok(None) => return Err(self.context.error(ErrorType::SyntaxError)),
                Err(e) => return Err(self.context.error(e))
            };
        } else {
            self.context.add_label(l.to_string())?
        }
        Ok(())
    }

    pub fn translate(&mut self, tokens: &mut Vec<Token>) -> Result<(), Error> {
        self.context.next_line();
        self.tokens = tokens.to_owned();
        self.tokens.reverse();
        while !self.tokens.is_empty() {
            if let Some(tok) = self.tokens.pop() {
                match tok {
                    Token::Directive(d) => self.process_directive(d)?,
                    Token::ConstLabel(l) => self.handle_label(&l)?,
                    Token::OpCode(op) => self.handle_opcodes(op)?,
                    Token::Invalid => return Err(self.context.error(ErrorType::InvalidLabel)),
                    _ => return Err(self.context.error(ErrorType::SyntaxError))
                }
            }
        }
        Ok(())
    }

    pub fn dump(&mut self) {
        println!();
        magenta_ln!("--=[ debug info ]=--");
        magenta_ln!("Origin            : {:02X}", self.origin);
        magenta_ln!("Total Lines       : {}", self.total_lines);
        magenta_ln!("Code Length       : {:02X}", self.context.current_pc() - self.origin);
//		magenta_ln!("Labels            : {:?}", self.context.labels);
//		magenta_ln!("Constants         : {:?}", self.context.constants);
//		magenta_ln!("Forward references: {:02X?}", self.context.forward_references);
        //magenta_ln!("Assembled         : {:02X?}", self.bytes);
    }
}