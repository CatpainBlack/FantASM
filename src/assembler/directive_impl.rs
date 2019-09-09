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
use std::io::Read;
use std::path::Path;

use ascii::AsAsciiStr;

use crate::assembler::{Assembler, Error};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::tokens::{Directive, OptionType, Token};
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::Token::{ConstLabel, Delimiter, Opt, StringLiteral};

pub trait Directives {
    fn set_origin(&mut self) -> Result<(), Error>;
    fn handle_string(&mut self, s: &str, terminator: Option<u8>) -> Result<(), Error>;
    fn handle_bytes(&mut self) -> Result<(), Error>;
    fn handle_words(&mut self) -> Result<(), Error>;
    fn handle_block(&mut self) -> Result<(), Error>;
    fn handle_hex(&mut self) -> Result<(), Error>;
    fn set_option(&mut self) -> Result<(), Error>;
    fn include_source_file(&mut self) -> Result<(), Error>;
    fn write_message(&mut self) -> Result<(), Error>;
    fn include_binary(&mut self) -> Result<(), Error>;
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

    fn handle_string(&mut self, s: &str, terminator: Option<u8>) -> Result<(), Error> {
        let ascii_string = match s.as_ascii_str() {
            Ok(a) => a,
            Err(e) => return Err(self.context.error_text(ErrorType::NonAscii, &e.to_string()))
        };
        self.emit(ascii_string.as_bytes())?;
        if terminator.is_some() {
            self.emit_byte(terminator.unwrap())?;
        }
        Ok(())
    }

    fn handle_bytes(&mut self) -> Result<(), Error> {
        let mut expect_comma = false;
        while !&self.tokens.is_empty() {
            if expect_comma {
                self.expect_token(Delimiter(Comma))?
            } else {
                let t = self.take_token()?;
                if let StringLiteral(s) = t {
                    self.tokens.pop();
                    self.handle_string(&s, None)?;
                    continue;
                } else {}
                self.tokens.push(t);
                match self.expr.parse(&mut self.context, &mut self.tokens, 0, 1) {
                    Ok(Some(n)) => {
                        if !(0..256).contains(&n) {
                            self.warn(ErrorType::IntegerOutOfRange);
                        }
                        self.emit(&[n as u8])?
                    }
                    Ok(None) => if let StringLiteral(s) = self.take_token()? {
                        println!("Ok(None)");
                        self.emit(s.into_bytes().as_slice())?;
                    } else {
                        println!("Hmm");
                        return Err(self.context.error(ErrorType::SyntaxError));
                    }
                    Err(e) => {
                        println!("Err({:?})", e.to_string());
                        return Err(self.context.error(e));
                    }
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
                match self.expr.parse(&mut self.context, &mut self.tokens, 0, 1) {
                    Ok(Some(n)) => {
                        if !(0..65536).contains(&n) {
                            self.warn(ErrorType::IntegerOutOfRange);
                        }
                        self.emit_word(n)?
                    }
                    Ok(None) => return Err(self.context.error(ErrorType::SyntaxError)),
                    Err(e) => {
                        println!("Err({:?})", e.to_string());
                        return Err(self.context.error(e));
                    }
                }
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
        self.emit(vec![fill; size].as_slice())?;
        Ok(())
    }

    fn handle_hex(&mut self) -> Result<(), Error> {
        if let StringLiteral(s) = self.take_token()? {
            let mut hex = s.as_str().as_bytes().to_vec();
            let mut bytes = vec![];
            while hex.len() > 0 {
                let lo = hex.pop().unwrap() as char;
                let mut hi = '0';
                if hex.len() > 0 {
                    hi = hex.pop().unwrap() as char;
                }
                let x = format!("{}{}", hi, lo);
                if let Ok(s) = u8::from_str_radix(&x, 16) {
                    bytes.insert(0, s);
                } else {
                    return Err(self.context.error(ErrorType::HexStringExpected));
                }
            }
            self.emit(&bytes)
        } else {
            Err(self.context.error(ErrorType::HexStringExpected))
        }
    }

    fn set_option(&mut self) -> Result<(), Error> {
        let o = self.take_token()?;
        let b = self.take_token()?;
        match (o, b) {
            (Opt(OptionType::Verbose), Token::Boolean(b)) => self.enable_console(b),
            (Opt(OptionType::CSpect), Token::Boolean(b)) => self.enable_cspect(b),
            (Opt(OptionType::Z80n), Token::Boolean(b)) => self.enable_z80n(b),
            (_, _) => return Err(self.context.error(ErrorType::InvalidOption))
        };
        Ok(())
    }

    fn include_source_file(&mut self) -> Result<(), Error> {
        let file_name = match self.take_token()? {
            StringLiteral(s) => s,
            ConstLabel(l) => l,
            _ => return Err(self.context.error(ErrorType::FileNotFound))
        };
        self.info(format!("Including file from {}", file_name).as_str());
        if self.context.is_included(&file_name) {
            return Err(self.context.error(ErrorType::MultipleIncludes));
        }
        self.first_pass(file_name.as_str())
    }

    fn write_message(&mut self) -> Result<(), Error> {
        if let StringLiteral(s) = self.take_token()? {
            println!("{}", s);
        }
        Ok(())
    }

    fn include_binary(&mut self) -> Result<(), Error> {
        let file_name = match self.take_token()? {
            StringLiteral(s) => s,
            ConstLabel(l) => l,
            _ => return Err(self.context.error(ErrorType::FileNotFound))
        };
        if !Path::new(&file_name).exists() {
            return Err(self.context.error(ErrorType::FileNotFound));
        }
        self.info(format!("Including binary file from {}", file_name).as_str());
        let mut b: Vec<u8> = vec![];
        let mut f = File::open(&file_name)?;
        let r = f.read_to_end(b.as_mut())? as isize;
        self.bank.append(&mut b);
        let pc = self.context.offset_pc(r);
        self.context.pc(pc);
        Ok(())
    }

    fn process_directive(&mut self, directive: Directive) -> Result<(), Error> {
        match directive {
            Directive::Org => self.set_origin(),
            Directive::Include => self.include_source_file(),
            Directive::Message => self.write_message(),
            Directive::Byte => self.handle_bytes(),
            Directive::Word => self.handle_words(),
            Directive::Opt => self.set_option(),
            Directive::Binary => self.include_binary(),
            Directive::Block => self.handle_block(),
            Directive::Macro => self.macros.begin_collect(&mut self.context, &mut self.tokens),
            Directive::StringZero => if let StringLiteral(s) = self.take_token()? {
                self.handle_string(&s, Some(0))
            } else {
                return Err(self.context.error_text(ErrorType::SyntaxError, "String expected"));
            }
            Directive::End => {
                if self.macros.collecting() {
                    self.macros.end_collect(&mut self.context)
                } else {
                    return Err(self.context.error(ErrorType::DanglingEnd));
                }
            }
            //Directive::Align => {}
            Directive::Hex => self.handle_hex(),
            _ => Err(self.context.error_text(ErrorType::UnhandledDirective, &format!("{:?}", directive)))
        }
        //Ok(())
    }
}