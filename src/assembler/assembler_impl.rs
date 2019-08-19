use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Range;

use crate::assembler::{Assembler, Error, ErrorLevel, TokenReader};
use crate::assembler::directive_impl::Directives;
use crate::assembler::error_impl::ErrorType;
use crate::assembler::expression_impl::ExpressionParser;
use crate::assembler::instruction_encoder::InstructionEncoder;
use crate::assembler::reg_pair::HighLow;
use crate::assembler::tokens::{AluOp, OpCode, Token};
use crate::assembler::tokens::Op::Equals;
use crate::assembler::tokens::RotOp::{Rl, Rlc, Rr, Rrc, Sla, Sll, Sra, Srl};
use crate::assembler::tokens::Token::{Label, Number, Operator};

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
        }
    }

    pub fn enable_console(&mut self) -> &mut Assembler {
        self.console_output = true;
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
        self.file_name.push(file_name.to_string());
        self.line_number.push(0);
        let file = File::open(file_name)?;
        let buf = BufReader::new(file);
        let mut reader = TokenReader::new(buf);
        reader.delimiters(",").operators("()*/+-<>=^&|");
        self.tokens.clear();
        loop {
            let tokens = reader.read_line()?;
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
        loop {
            if let Some((dest, label, relative)) = self.forward_references.pop() {
                let addr = self.get_label_or_constant_value(label.as_str())?;
                let index = dest as usize - self.origin as usize;
                if relative {
                    let offset = addr - (dest + 1) as isize;
                    self.bytes[index] = offset as u8;
                } else {
                    self.bytes[index] = addr.lo();
                    self.bytes[index + 1] = addr.hi();
                }
            } else {
                break;
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
        if let Some(address) = self.labels.get(name) {
            return Ok(address.clone());
        }
        if let Some(address) = self.constants.get(name) {
            return Ok(address.clone());
        }
        Err(self.error(ErrorType::LabelNotFound))
    }


    fn cur_file(&self) -> String {
        self.file_name.last().unwrap_or(&String::from("fantasm")).to_string()
    }

    fn cur_line(&self) -> isize {
        self.line_number.last().unwrap_or(&0isize).clone()
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
            line_no: self.line_number.last().unwrap().clone(),
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
            Label(s) => {
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
        let label_name = name.replace(":", "").to_string();

        if let Some(a) = self.constants.get(label_name.as_str()) {
            addr = a.clone() as u16;
        } else if let Some(a) = self.labels.get(label_name.as_str()) {
            addr = a.clone() as u16;
        } else {
            self.forward_references.push(((self.current_pc + pc_offset) as u16, label_name, relative));
        }
        return addr;
    }

    pub fn get_address(&mut self, pc_offset: isize) -> Result<Option<u16>, Error> {
        let addr = match self.tokens.clone().last() {
            Some(Label(s)) => self.try_resolve_label(s.as_str(), pc_offset, false),
            Some(Number(n)) => if (0isize..65536).contains(&n) {
                n.clone() as u16
            } else {
                self.warn(ErrorType::AddressTruncated);
                n.clone() as u16
            }
            _ => 0
        };
        Ok(Some(addr))
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

    pub fn expect_number(&mut self, in_range: Range<isize>) -> Result<isize, Error> {
        if let Number(n) = self.next_token()? {
            if in_range.contains(&n) {
                return Ok(n);
            }
            return Err(self.error(ErrorType::IntegerOutOfRange));
        }
        Err(self.error(ErrorType::IntegerExpected))
    }


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
            OpCode::Call => self.call()?,
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
            //_ => return Err(Error::fatal(format!("Unhandled opcode: {:?}", op).as_str(), self.line_number))
        };
        self.emit(bytes);
        Ok(())
    }

    fn handle_label(&mut self, l: &str) -> Result<(), Error> {
        if self.next_token_is(&Operator(Equals)) {
            self.tokens.pop();
            match self.expr.parse(&mut self.tokens, &mut self.constants, &mut self.labels) {
                Ok(Some(n)) => self.add_constant(l.to_string(), n)?,
                Ok(None) => return Err(self.error(ErrorType::SyntaxError)),
                Err(e) => return Err(self.error(e))
            };
        } else {
            self.add_label(l.to_string())?
        }
        Ok(())
    }

    pub fn translate(&mut self, tokens: Vec<Token>) -> Result<(), Error> {
        let len = self.line_number.len() - 1;
        self.line_number[len] += 1;
        self.tokens = tokens.clone();
        self.tokens.reverse();
        while !self.tokens.is_empty() {
            if let Some(tok) = self.tokens.pop() {
                match tok {
                    Token::Directive(d) => self.process_directive(d)?,
                    Token::Label(l) => self.handle_label(&l)?,
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
        //magenta_ln!("Forward references: {:02X?}", self.forward_references);
        //magenta_ln!("Assembled         : {:02X?}", self.bytes);
    }
}