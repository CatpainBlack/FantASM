use crate::assembler::Assembler;
use crate::assembler::error::Error;
use crate::assembler::error_type::ErrorType;
use crate::assembler::reg_pair::HighLow;
use crate::assembler::tokens::Token;

pub trait Emitter {
	fn emit(&mut self, b: &[u8]) -> Result<(), Error>;
	fn emit_byte(&mut self, b: u8) -> Result<(), Error>;
	fn emit_word(&mut self, word: isize) -> Result<(), Error>;
	fn emit_instr(&mut self, prefix: Option<u8>, instr: u8, expr: &[Token], byte: bool) -> Result<(), Error>;
}

impl Emitter for Assembler {
	fn emit(&mut self, b: &[u8]) -> Result<(), Error> {
		let pc = self.context.offset_pc(b.len() as isize);
		if pc > 65535 {
			self.warn(ErrorType::PCOverflow)
		}
		self.context.pc(pc);
		self.context.result(self.bank.append(&mut b.to_vec()))
	}

	fn emit_byte(&mut self, b: u8) -> Result<(), Error> {
		let pc = self.context.offset_pc(1);
		if pc > 65535 {
			self.warn(ErrorType::PCOverflow)
		}
		self.context.pc(pc);
		self.context.result(self.bank.push(b))?;
		Ok(())
	}

	fn emit_word(&mut self, word: isize) -> Result<(), Error> {
		if word < 0 || word > 65535 {
			self.warn(ErrorType::WordTruncated);
		}
		let w = word as u16;
		let pc = self.context.offset_pc(2);
		if pc > 65535 {
			self.warn(ErrorType::PCOverflow)
		}
		self.context.pc(pc);
		self.context.result(self.bank.push(w.lo()))?;
		self.context.result(self.bank.push(w.hi()))
	}

	fn emit_instr(&mut self, prefix: Option<u8>, instr: u8, expr: &[Token], byte: bool) -> Result<(), Error> {
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
}