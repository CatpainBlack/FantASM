/*
Copyright (c) 2019, Guy Black
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

The views and conclusions contained in the software and documentation are those
of the authors and should not be interpreted as representing official policies,
either expressed or implied, of the FantASM project.
*/

use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Range;

use crate::assembler::{Assembler, Error, ErrorLevel, TokenReader};
use crate::assembler::bank_impl::Bank;
use crate::assembler::directive_impl::Directives;
use crate::assembler::error_impl::ErrorType;
use crate::assembler::error_impl::ErrorType::SyntaxError;
use crate::assembler::expression_impl::ExpressionParser;
use crate::assembler::instruction_encoder::InstructionEncoder;
use crate::assembler::macro_impl::MacroHandler;
use crate::assembler::reg_pair::HighLow;
use crate::assembler::tokens::{AluOp, OpCode, Token};
use crate::assembler::tokens::Directive::End;
use crate::assembler::tokens::Op::Equals;
use crate::assembler::tokens::RotOp::{Rl, Rlc, Rr, Rrc, Sla, Sll, Sra, Srl};
use crate::assembler::tokens::Token::Operator;

impl Assembler {
    pub fn new() -> Assembler {
        let context = Default::default();
        Assembler {
            context,
            macros: MacroHandler::new(),
            tokens: vec![],
            origin: 0,
            bank: Bank::new(),
            console_output: false,
            total_lines: 0,
            expr: ExpressionParser::new(),
            z80n_enabled: false,
            cspect_enabled: false,
            debug: false,
            collect_macro: false,
            warnings: vec![],
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

    pub fn enable_debug(&mut self, enabled: bool) -> &mut Assembler {
        self.debug = enabled;
        self
    }

    fn write_status(&self) {
        if self.console_output {
            if self.num_warnings() > 0 {
                cyan_ln!("Completed with {} warning(s)",self.num_warnings());
            }
            // else {
//                dark_green_ln!("Done");
//            }
            self.display_warnings();
        }
    }

    pub fn assemble(&mut self, file_name: &str) -> Result<(), Error> {
        self.warnings.clear();
        if self.console_output { green_ln!("First pass .... "); }

        self.first_pass(file_name)?;
        self.write_status();

        self.warnings.clear();
        if self.console_output { green_ln!("Second pass ... "); }

        self.second_pass()?;
        self.write_status();

        if self.debug {
            self.dump();
            self.macros.dump();
        }

        Ok(())
    }


    pub(crate) fn first_pass(&mut self, file_name: &str) -> Result<(), Error> {
        self.collect_macro = false;
        self.context.enter(file_name);
        let file = File::open(file_name)?;
        let buf = BufReader::new(file);
        let mut reader = TokenReader::new(buf);
        reader.delimiters(",").operators("()*/+-<>=^&|");
        self.tokens.clear();
        loop {
            let tokens = &mut reader.read_line()?;
            self.total_lines += 1;
            if tokens.first() == Some(&Token::EndOfFile) {
                break;
            }
            if self.macros.collecting() && tokens.first() != Some(&Token::Directive(End)) {
                self.macros.collect(&mut self.context, tokens)?;
            } else {
                self.translate(tokens)?;
            }
        }
        self.context.leave();
        Ok(())
    }

    pub fn second_pass(&mut self) -> Result<(), Error> {
        while let Some(mut fwd_ref) = self.context.next_forward_ref() {
            let mut data: isize;
            if fwd_ref.is_expression {
                self.context.label_context = fwd_ref.label;
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
        self.warnings.push(format!("[{} : {}] Warning: {}", self.context.current_file_name(), self.context.current_line_number(), t.to_string()))
    }

    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }

    pub fn display_warnings(&self) {
        if self.console_output {
            for warning in &self.warnings {
                cyan_ln!("{}",warning);
            }
        }
    }

    pub fn info(&mut self, m: &str) {
        if self.console_output {
            yellow_ln!("[{}:{}] {}",  self.context.current_file_name(), self.context.current_line_number(), m);
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
        let addr = match self.expr.parse(&mut self.context, &mut self.tokens, 1, 1, true) {
            Ok(Some(n)) => n,
            Ok(None) => 0,
            Err(e) => return Err(self.context.error(e)),
        };
        let pc = (self.context.offset_pc(2)) as isize;
        Ok((addr - pc) as u8)
    }

    pub(crate) fn expect_byte(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..256, 1, ErrorType::ByteTruncated, instr_size)
    }

    pub(crate) fn expect_word(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..65536, 2, ErrorType::WordTruncated, instr_size)
    }

    fn expect_number_in_range(&mut self, range: Range<isize>, count: isize, error_type: ErrorType, instr_size: isize) -> Result<isize, Error> {
        match self.expr.parse(&mut self.context, &mut self.tokens, instr_size, count, false) {
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

    pub fn take_token(&mut self) -> Result<Token, Error> {
        if let Some(tok) = self.tokens.pop() {
            return Ok(tok);
        }
        Err(self.context.error(ErrorType::UnexpectedEndOfLine))
    }

    pub fn next_token_is(&mut self, tok: &Token) -> bool {
        if let Some(t) = self.tokens.last() {
            t == tok
        } else {
            false
        }
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

    pub(crate) fn emit_instr(&mut self, prefix: Option<u8>, instr: u8, expr: &[Token], byte: bool) -> Result<(), Error> {
        if prefix.is_some() {
            self.emit_byte(prefix.unwrap())?;
        }
        self.emit_byte(instr)?;
        let a = match self.expr.parse(&mut self.context, &mut expr.to_vec(), 0, 2, false) {
            Ok(Some(addr)) => addr,
            Ok(None) => 0,
            Err(e) => return Err(self.context.error(e))
        };
        if byte {
            return self.emit_byte(a as u8);
        }
        self.emit_word(a)
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
                Ok(())
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
            match self.expr.parse(&mut self.context, &mut self.tokens, 0, -1, false) {
                Ok(Some(n)) => self.context.add_constant(l.to_string(), n)?,
                Ok(None) => return Err(self.context.error(ErrorType::SyntaxError)),
                Err(e) => return Err(self.context.error(e))
            };
            if !self.tokens.is_empty() {
                self.tokens.clear();
                self.warn(ErrorType::ExtraCharacters)
            }
        } else {
            self.context.add_label(l.to_string())?
        }
        Ok(())
    }

    pub fn translate(&mut self, tokens: &mut Vec<Token>) -> Result<(), Error> {
        if !self.macros.expanding() {
            self.context.next_line();
        }
        self.tokens = tokens.to_owned();
        self.tokens.reverse();
        while !self.tokens.is_empty() {
            self.context.init_asm_pc();
            if let Some(tok) = self.tokens.pop() {
                match &tok {
                    Token::ConstLabel(l) => if self.macros.macro_defined(l) {
                        self.macros.begin_expand(&mut self.context, l, &mut self.tokens)?;
                        while let Some(line) = self.macros.expand() {
                            self.translate(&mut line.clone())?
                        }
                    } else {
                        self.handle_label(l)?
                    }
                    Token::Directive(d) => self.process_directive(*d)?,
                    Token::OpCode(op) => self.handle_opcodes(op.clone())?,
                    Token::Invalid => return Err(self.context.error(ErrorType::InvalidLabel)),
                    _ => {
                        return Err(self.context.error(ErrorType::SyntaxError));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn dump(&mut self) {
        magenta_ln!("--=[ debug info ]=--");
        magenta_ln!("Origin            : {} [0x{:02X}]", self.origin,self.origin);
        magenta_ln!("Total Lines       : {}", self.total_lines);
        magenta_ln!("Code Length       : {}", self.bank.as_slice().len());
        self.context.dump();
    }
}