use crate::assembler::{Assembler, Error};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::{Directive, OptionType, Token};
use crate::assembler::tokens::Token::{Delimiter, ConstLabel, StringLiteral, Opt};
use crate::assembler::reg_pair::HighLow;

pub trait Directives {
	fn set_origin(&mut self) -> Result<(), Error>;
	fn handle_bytes(&mut self) -> Result<(), Error>;
	fn handle_words(&mut self) -> Result<(), Error>;
	fn handle_block(&mut self) -> Result<(), Error>;
	fn set_option(&mut self) -> Result<(), Error>;
	fn include_source_file(&mut self) -> Result<(), Error>;
	fn write_message(&mut self) -> Result<(), Error>;
	fn process_directive(&mut self, directive: Directive) -> Result<(), Error>;
}

impl Directives for Assembler {
	fn set_origin(&mut self) -> Result<(), Error> {
		match self.expr.parse(&mut self.context, &mut self.tokens, 0, -1) {
			Ok(Some(mut o)) => {
				if o > 65535 {
					o = o & 0xFFFF;
					self.warn(ErrorType::AddressTruncated);
				}
				self.origin = o
			}
			Ok(None) => return Err(self.context.error(ErrorType::SyntaxError)),
			Err(e) => return Err(self.context.error(e))
		}
		self.context.pc(self.origin);
		Ok(())
	}

	fn handle_bytes(&mut self) -> Result<(), Error> {
		let mut expect_comma = false;
		while !self.tokens.is_empty() {
			if expect_comma {
				self.expect_token(Delimiter(Comma))?
			} else {
				match self.expr.parse(&mut self.context, &mut self.tokens, 0, 1) {
					Ok(Some(n)) => {
						if !(0..256).contains(&n) {
							self.warn(ErrorType::IntegerOutOfRange);
						}
						self.emit(vec![n as u8])
					}
					Ok(None) => if let StringLiteral(s) = self.next_token()? {
						self.emit(s.into_bytes());
					} else {
						return Err(self.context.error(ErrorType::SyntaxError));
					}
					Err(e) => return Err(self.context.error(e))
				}
			}
			expect_comma = !expect_comma;
		}
		Ok(())
	}

	fn handle_words(&mut self) -> Result<(), Error> {
		let mut expect_comma = false;
		while !self.tokens.is_empty() {
			if expect_comma {
				self.expect_token(Delimiter(Comma))?
			} else {
				let word = self.expect_word(0)?;
				self.emit(vec![word.lo(), word.hi()]);
			}
			expect_comma = !expect_comma;
		}
		Ok(())
	}

	fn handle_block(&mut self) -> Result<(), Error> {
		let size = self.expect_word(-1)? as usize;
		let mut fill = 0u8;
		if self.next_token_is(&Delimiter(Comma)) {
			self.tokens.pop();
			fill = self.expect_byte(-1)? as u8;
		}
		self.emit(vec![fill; size]);
		Ok(())
	}

	fn set_option(&mut self) -> Result<(), Error> {
		let o = self.next_token()?;
		let b = self.next_token()?;
		match (o, b) {
			(Opt(OptionType::Verbose), Token::Boolean(b)) => self.enable_console(b),
			(Opt(OptionType::CSpect), Token::Boolean(b)) => self.enable_cspect(b),
			(Opt(OptionType::Z80n), Token::Boolean(b)) => self.enable_z80n(b),
			(_, _) => return Err(self.context.error(ErrorType::InvalidOption))
		};
		Ok(())
	}

	fn include_source_file(&mut self) -> Result<(), Error> {
		let file_name = match self.next_token()? {
			StringLiteral(s) => s,
			ConstLabel(l) => l,
			_ => return Err(self.context.error(ErrorType::FileNotFound))
		};
		self.info(format!("Including file from {}", file_name).as_str());
		self.first_pass(file_name.as_str())
	}

	fn write_message(&mut self) -> Result<(), Error> {
		if let StringLiteral(s) = self.next_token()? {
			println!("{}", s);
		}
		Ok(())
	}

	fn process_directive(&mut self, directive: Directive) -> Result<(), Error> {
		match directive {
			Directive::Org => self.set_origin()?,
			Directive::Include => self.include_source_file()?,
			Directive::Message => self.write_message()?,
			Directive::Byte => self.handle_bytes()?,
			Directive::Word => self.handle_words()?,
			Directive::Opt => self.set_option()?,
			//Directive::Binary => {}
			Directive::Block => self.handle_block()?,
			//Directive::Hex => {}
			_ => {
				let line_no = self.context.current_line_number();
				return Err(Error::fatal(&format!("Unhandled directive: {:?}", directive), line_no));
			}
		}
		Ok(())
	}
}