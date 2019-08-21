use std::ops::Range;

use crate::assembler::{Assembler, Error};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::Directive;
use crate::assembler::tokens::Token::{Delimiter, Label, Number, StringLiteral};

pub trait Directives {
    fn set_origin(&mut self) -> Result<(), Error>;
    fn handle_data(&mut self, range: Range<isize>) -> Result<(), Error>;
    fn include_file(&mut self) -> Result<(), Error>;
    fn write_message(&mut self) -> Result<(), Error>;
    fn process_directive(&mut self, directive: Directive) -> Result<(), Error>;
}

impl Directives for Assembler {
    fn set_origin(&mut self) -> Result<(), Error> {
        match self.expr.parse(&mut self.tokens, &mut self.constants, &mut self.labels) {
            Ok(Some(mut o)) => {
                if o > 65535 {
                    o = o & 0xFFFF;
                    self.warn(ErrorType::AddressTruncated);
                }
                self.origin = o
            }
            Ok(None) => return Err(self.error(ErrorType::SyntaxError)),
            Err(e) => return Err(self.error(e))
        }
        self.current_pc = self.origin;
        Ok(())
    }

    fn handle_data(&mut self, range: Range<isize>) -> Result<(), Error> {
        let mut expect_comma = false;
        while !self.tokens.is_empty() {
            if expect_comma {
                self.expect_token(Delimiter(Comma))?
            } else {
                match self.next_token()? {
                    StringLiteral(s) => {
                        self.emit(s.into_bytes());
                    }
                    Number(n) => if range.contains(&n) {
                        self.emit(vec![n as u8])
                    } else {
                        return Err(self.error(ErrorType::IntegerOutOfRange));
                    }
                    _ => return Err(self.error(ErrorType::SyntaxError))
                }
            }
            expect_comma = !expect_comma;
        }
        Ok(())
    }

    fn include_file(&mut self) -> Result<(), Error> {
        let file_name = match self.next_token()? {
            StringLiteral(s) => s,
            Label(l) => l,
            _ => return Err(self.error(ErrorType::FileNotFound))
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
            Directive::Include => self.include_file()?,
            Directive::Message => self.write_message()?,
            Directive::Byte => self.handle_data(0..256)?,
            //Directive::Binary => {}
            //Directive::Word => {}
            //Directive::Block => {}
            //Directive::Hex => {}
            _ => {
                let line_no = self.line_number.last().unwrap_or(&0);
                return Err(Error::fatal(&format!("Unhandled directive: {:?}", directive), *line_no));
            }
        }
        Ok(())
    }
}