use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Range;

use crate::assembler::{Assembler, Error, ErrorLevel, TokenReader, ForwardReference};
use crate::assembler::directive_impl::Directives;
use crate::assembler::error_impl::ErrorType;
use crate::assembler::expression_impl::ExpressionParser;
use crate::assembler::instruction_encoder::InstructionEncoder;
use crate::assembler::tokens::{AluOp, OpCode, Token};
use crate::assembler::tokens::Op::Equals;
use crate::assembler::tokens::RotOp::{Rl, Rlc, Rr, Rrc, Sla, Sll, Sra, Srl};
use crate::assembler::tokens::Token::{ConstLabel, Number, Operator, AddressIndirect, ConstLabelIndirect};

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            line_number: vec![],
            tokens: vec![],
            origin: 0,
            current_pc: 0,
            labels: HashMap::new(),
            constants: HashMap::new(),
            bytes: vec![],
            forward_references: vec![],
            file_name: vec![],
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
        self.expr.init(&mut self.labels, &mut self.constants, &mut self.forward_references);
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
        self.file_name.push(file_name.to_string());
        self.line_number.push(0);
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
        self.file_name.pop();
        self.line_number.pop();
        Ok(())
    }

    pub fn second_pass(&mut self) -> Result<(), Error> {
        while let Some(mut fwd_ref) = self.forward_references.pop() {
            let mut data: isize;
            if fwd_ref.is_expression {
                data = match self.expr.eval(fwd_ref.expression.as_mut()) {
                    Ok(n) => n,
                    Err(e) => return Err(self.error(e)),
                }
            } else {
                data = self.get_label_or_constant_value(fwd_ref.label.as_str())?;
            }
            let index = fwd_ref.pc as usize - self.origin as usize;
            if fwd_ref.is_relative {
                let offset = data - (fwd_ref.pc + 1) as isize;
                self.bytes[index] = offset as u8;
            } else {
                for d in 0..fwd_ref.byte_count as usize {
                    self.bytes[index + d] = (data & 0xff) as u8;
                    data = data >> 8;
                }
                // fixup the z80n "push nnnn" endiannnessssss
                if self.z80n_enabled && fwd_ref.byte_count == 2 && index > 1 && self.bytes[index - 2] == 0xed && self.bytes[index - 1] == 0x8a {
                    let b = self.bytes[index + 1];
                    self.bytes[index + 1] = self.bytes[index];
                    self.bytes[index] = b;
                }
            }
        }
        Ok(())
    }

    pub fn save_raw(&mut self, file_name: &str) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        file.write_all(self.bytes.as_slice())?;
        Ok(())
    }

    fn get_label_or_constant_value(&mut self, name: &str) -> Result<isize, Error> {
        if let Some(&address) = self.labels.get(name) {
            return Ok(address);
        }
        if let Some(&address) = self.constants.get(name) {
            return Ok(address);
        }
        Err(self.error(ErrorType::LabelNotFound))
    }


    fn cur_file(&self) -> String {
        self.file_name.last().unwrap_or(&String::from("<none>")).to_string()
    }

    fn cur_line(&self) -> isize {
        *self.line_number.last().unwrap_or(&0isize)
    }

    pub fn warn(&mut self, t: ErrorType) {
        if self.console_output {
            cyan_ln!("[{} : {}] Warning: {}", self.cur_file(), self.cur_line(), t.to_string());
        }
    }

    pub fn info(&mut self, m: &str) {
        if self.console_output {
            yellow_ln!("[{} : {}] {}",  self.cur_file(), self.cur_line(),m);
        }
    }

    pub fn error(&mut self, t: ErrorType) -> Error {
        let mut e = Error {
            line_no: *self.line_number.last().unwrap_or(&0),
            message: t.to_string(),
            level: ErrorLevel::Fatal,
            file_name: "".to_string(),
        };
        e.file_name = self.cur_file();
        e
    }

    pub fn relative(&mut self) -> Result<u8, Error> {
        match self.next_token()? {
            Number(n) => Ok((n - (self.current_pc as isize + 2)) as u8),
            ConstLabel(s) => {
                let mut addr = self.try_resolve_label(&s, 1, true) as isize;
                let pc = (self.current_pc + 2) as isize;
                if addr == 0 {
                    addr = pc;
                }
                Ok(((addr - pc) as u16) as u8)
            }
            _ => Err(self.error(ErrorType::SyntaxError))
        }
    }

    pub(crate) fn try_resolve_label(&mut self, name: &str, pc_offset: isize, relative: bool) -> u16 {
        let mut addr = 0;
        let label_name = &*name.replace(":", "");

        if let Some(a) = self.constants.get(label_name) {
            addr = *a as u16;
        } else if let Some(a) = self.labels.get(label_name) {
            addr = *a as u16;
        } else {
            self.forward_references.push(ForwardReference {
                is_expression: false,
                pc: self.current_pc + pc_offset,
                label: label_name.to_string(),
                expression: vec![],
                is_relative: relative,
                byte_count: 2,
            });
        }
        return addr;
    }

    pub(crate) fn expect_byte(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..256, 1, ErrorType::ByteTrunctated, instr_size)
    }

    pub(crate) fn expect_word(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..65536, 2, ErrorType::WordTruncated, instr_size)
    }

    fn expect_number_in_range(&mut self, range: Range<isize>, count: isize, error_type: ErrorType, instr_size: isize) -> Result<isize, Error> {
        match self.expr.parse(&mut self.tokens, self.current_pc + instr_size, count) {
            Ok(Some(n)) => {
                if !range.contains(&n) {
                    self.warn(error_type);
                }
                Ok(n)
            }
            Ok(None) => return Err(self.error(ErrorType::SyntaxError)),
            Err(e) => return Err(self.error(e))
        }
    }

    pub(crate) fn decode_number(&mut self, token: &Token, pc_offset: isize) -> Result<Option<isize>, Error> {
        match &token {
            Number(n) => Ok(Some(*n)),
            AddressIndirect(a) => Ok(Some(*a as isize)),
            ConstLabelIndirect(l) => Ok(Some(self.try_resolve_label(l, pc_offset, false) as isize)),
            ConstLabel(l) => if let Some(n) = self.constants.get(l) {
                Ok(Some(*n))
            } else {
                Err(self.error(ErrorType::BadConstant))
            }
            _ => Ok(None)
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        if let Some(tok) = self.tokens.pop() {
            return Ok(tok);
        }
        Err(self.error(ErrorType::UnexpectedEndOfLine))
    }

    pub fn next_token_is(&mut self, tok: &Token) -> bool {
        if let Some(t) = self.tokens.last() {
            return t == tok;
        }
        return false;
    }

    pub fn expect_token(&mut self, tok: Token) -> Result<(), Error> {
        let t = self.next_token()?;
        if t != tok {
            return Err(self.error(ErrorType::SyntaxError));
        }
        Ok(())
    }

//    pub fn expect_number(&mut self, in_range: Range<isize>) -> Result<isize, Error> {
//        if let Number(n) = self.next_token()? {
//            if in_range.contains(&n) {
//                return Ok(n);
//            }
//            return Err(self.error(ErrorType::IntegerOutOfRange));
//        }
//        Err(self.error(ErrorType::IntegerExpected))
//    }


    fn add_label(&mut self, name: String) -> Result<(), Error> {
        if self.labels.contains_key(name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.labels.insert(name, self.current_pc);
        Ok(())
    }

    fn add_constant(&mut self, name: String, value: isize) -> Result<(), Error> {
        if self.constants.contains_key(name.as_str()) {
            return Err(self.error(ErrorType::LabelOrConstantExists));
        }
        self.constants.insert(name, value);
        Ok(())
    }

    pub(crate) fn emit(&mut self, mut b: Vec<u8>) {
        if self.current_pc + b.len() as isize > 65535 {
            self.warn(ErrorType::PCOverflow)
        }
        self.current_pc += b.len() as isize;
        self.bytes.append(&mut b);
    }

    fn handle_opcodes(&mut self, op: OpCode) -> Result<(), Error> {
        let bytes = match op {
            OpCode::Nop => vec![0],
            OpCode::Adc => self.alu_op_r(AluOp::Adc, 1, 0)?,
            OpCode::Add => self.alu_op_r(AluOp::Add, 0, 1)?,
            OpCode::And => self.alu_op(AluOp::And)?,
            OpCode::Bit => self.bit_res_set(1)?,
            OpCode::Call => self.call_jp(1, 5)?,
            OpCode::Ccf => vec![0x3F],
            OpCode::Cp => self.alu_op(AluOp::Cp)?,
            OpCode::Cpd => vec![0xED, 0xA9],
            OpCode::Cpdr => vec![0xED, 0xB9],
            OpCode::Cpi => vec![0xED, 0xA1],
            OpCode::Cpir => vec![0xED, 0xB1],
            OpCode::Cpl => vec![0x2F],
            OpCode::Daa => vec![0x27],
            OpCode::Dec => self.inc_dec(1)?,
            OpCode::Di => vec![0xF3],
            OpCode::Djnz => vec![0x10, self.relative()?],
            OpCode::Ei => vec![0xFB],
            OpCode::Ex => self.ex()?,
            OpCode::Exx => vec![0xD9],
            OpCode::Halt => vec![0x76],
            OpCode::Im => self.im()?,
            OpCode::In => self.io_op(3)?,
            OpCode::Inc => self.inc_dec(0)?,
            OpCode::Ind => vec![0xED, 0xAA],
            OpCode::Indr => vec![0xED, 0xBA],
            OpCode::Ini => vec![0xED, 0xA2],
            OpCode::Inir => vec![0xED, 0xB2],
            OpCode::Jr => self.jr()?,
            OpCode::Jp => self.jp()?,
            OpCode::Ld => self.load()?,
            OpCode::Ldd => vec![0xED, 0xA8],
            OpCode::Lddr => vec![0xED, 0xB8],
            OpCode::Ldi => vec![0xED, 0xA0],
            OpCode::Ldir => vec![0xED, 0xB0],
            OpCode::Neg => vec![0xED, 0x44],
            OpCode::Or => self.alu_op(AluOp::Or)?,
            OpCode::Otdr => vec![0xED, 0xBB],
            OpCode::Otir => vec![0xED, 0xB3],
            OpCode::Out => self.io_op(2)?,
            OpCode::Outd => vec![0xED, 0xAB],
            OpCode::Outi => vec![0xED, 0xA3],
            OpCode::Pop => self.push_pop(1)?,
            OpCode::Push => self.push_pop(5)?,
            OpCode::Res => self.bit_res_set(2)?,
            OpCode::Ret => self.ret()?,
            OpCode::Reti => vec![0xED, 0x4D],
            OpCode::Retn => vec![0xED, 0x45],
            OpCode::Rl => self.rot(Rl)?,
            OpCode::Rla => vec![0x17],
            OpCode::Rlc => self.rot(Rlc)?,
            OpCode::Rlca => vec![0x07],
            OpCode::Rld => vec![0xED, 0x6F],
            OpCode::Rr => self.rot(Rr)?,
            OpCode::Rra => vec![0x1F],
            OpCode::Rrc => self.rot(Rrc)?,
            OpCode::Rrca => vec![0x0F],
            OpCode::Rrd => vec![0xED, 0x67],
            OpCode::Rst => self.rst()?,
            OpCode::Sbc => self.alu_op_r(AluOp::Sbc, 1, 1)?,
            OpCode::Scf => vec![0x37],
            OpCode::Set => self.bit_res_set(3)?,
            OpCode::Sla => self.rot(Sla)?,
            OpCode::Sll => self.rot(Sll)?,
            OpCode::Sra => self.rot(Sra)?,
            OpCode::Srl => self.rot(Srl)?,
            OpCode::Sub => self.alu_op(AluOp::Sub)?,
            OpCode::Xor => self.alu_op(AluOp::Xor)?,
            _ => if let Some(code) = self.encode_z80n(&op)? {
                code
            } else if let Some(code) = self.encode_cspect(&op)? {
                code
            } else {
                return Err(self.error(ErrorType::InvalidInstruction));
            }
        };
        self.emit(bytes);
        Ok(())
    }

    fn encode_cspect(&mut self, op: &OpCode) -> Result<Option<Vec<u8>>, Error> {
        let code = match op {
            OpCode::Break => Some(vec![0xDD, 1]),
            OpCode::Exit => Some(vec![0xDD, 0]),
            _ => None
        };
        if code.is_some() && !self.cspect_enabled {
            Err(self.error(ErrorType::CSpectDisabled))
        } else {
            Ok(code)
        }
    }

    fn encode_z80n(&mut self, op: &OpCode) -> Result<Option<Vec<u8>>, Error> {
        let code = match op {
            OpCode::Ldix => Some(vec![0xED, 0xA4]),
            OpCode::Ldws => Some(vec![0xED, 0xA5]),
            OpCode::Ldirx => Some(vec![0xED, 0xB4]),
            OpCode::Lddx => Some(vec![0xED, 0xAC]),
            OpCode::Lddrx => Some(vec![0xED, 0xBC]),
            OpCode::Ldpirx => Some(vec![0xED, 0xB7]),
            OpCode::Outinb => Some(vec![0xED, 0x90]),
            OpCode::Mul => Some(self.mul()?),
            OpCode::Swapnib => Some(vec![0xED, 0x23]),
            OpCode::Mirror => Some(vec![0xED, 0x24]),
            OpCode::Nextreg => Some(self.next_reg()?),
            OpCode::Pixeldn => Some(vec![0xED, 0x93]),
            OpCode::Pixelad => Some(vec![0xED, 0x94]),
            OpCode::Setae => Some(vec![0xED, 0x95]),
            OpCode::Test => Some(vec![0xED, 0x27, self.expect_byte(2)? as u8]),
            _ => None
        };
        if code.is_some() && !self.z80n_enabled {
            Err(self.error(ErrorType::Z80NDisabled))
        } else {
            Ok(code)
        }
    }

    fn handle_label(&mut self, l: &str) -> Result<(), Error> {
        if self.next_token_is(&Operator(Equals)) {
            self.tokens.pop();
            match self.expr.parse(&mut self.tokens, self.current_pc, -1) {
                Ok(Some(n)) => self.add_constant(l.to_string(), n)?,
                Ok(None) => return Err(self.error(ErrorType::SyntaxError)),
                Err(e) => return Err(self.error(e))
            };
        } else {
            self.add_label(l.to_string())?
        }
        Ok(())
    }

    pub fn translate(&mut self, tokens: &mut Vec<Token>) -> Result<(), Error> {
        let len = self.line_number.len() - 1;
        self.line_number[len] += 1;
        self.tokens = tokens.to_owned();
        self.tokens.reverse();
        while !self.tokens.is_empty() {
            if let Some(tok) = self.tokens.pop() {
                match tok {
                    Token::Directive(d) => self.process_directive(d)?,
                    Token::ConstLabel(l) => self.handle_label(&l)?,
                    Token::OpCode(op) => self.handle_opcodes(op)?,
                    Token::Invalid => return Err(self.error(ErrorType::InvalidLabel)),
                    _ => return Err(self.error(ErrorType::SyntaxError))
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
        magenta_ln!("Code Length       : {:02X}", self.current_pc - self.origin);
        magenta_ln!("Labels            : {:?}", self.labels);
        magenta_ln!("Constants         : {:?}", self.constants);
        magenta_ln!("Forward references: {:02X?}", self.forward_references);
        //magenta_ln!("Assembled         : {:02X?}", self.bytes);
    }
}