use std::fs::File;
use std::io::Read;
use std::path::Path;

use ascii::AsAsciiStr;

use crate::assembler::{Assembler, Error, IfBlock};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::IfBlock::{Else, If, SkipEnd};
use crate::assembler::tokens::{Directive, OptionType, Token};
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::Op::Equals;
use crate::assembler::tokens::Token::{ConstLabel, Delimiter, Operator, Opt, StringLiteral};
use crate::assembler::zx_ascii::ZXAscii;

pub trait Directives {
    fn set_origin(&mut self) -> Result<(), Error>;
    fn handle_string(&mut self, s: &str, terminator: Option<u8>) -> Result<(), Error>;
    fn handle_bytes(&mut self, terminator: Option<u8>) -> Result<(), Error>;
    fn handle_words(&mut self) -> Result<(), Error>;
    fn handle_block(&mut self) -> Result<(), Error>;
    fn handle_hex(&mut self) -> Result<(), Error>;
    fn set_option(&mut self) -> Result<(), Error>;
    fn locate_file(&mut self, file_name: &str) -> Result<String, Error>;
    fn include_source_file(&mut self) -> Result<(), Error>;
    fn write_message(&mut self) -> Result<(), Error>;
    fn include_binary(&mut self) -> Result<(), Error>;
    fn process_if(&mut self) -> Result<(), Error>;
    fn process_if_def(&mut self) -> Result<(), Error>;
    fn process_endif(&mut self) -> Result<(), Error>;
    fn process_else(&mut self) -> Result<(), Error>;
    fn process_global(&mut self) -> Result<(), Error>;
    fn process_directive(&mut self, directive: Directive) -> Result<(), Error>;
}

impl Directives for Assembler {
    fn set_origin(&mut self) -> Result<(), Error> {
        match self.expr.parse(&mut self.context, &mut self.tokens, 0, -1, false) {
            Ok(Some(mut o)) => {
                if o > 65535 {
                    o = o & 0xFFFF;
                    self.warn(ErrorType::AddressTruncated);
                }
                self.origin = o
            }
            Ok(None) => return Err(self.context.error(ErrorType::BadExpression)),
            Err(e) => return Err(self.context.error(e))
        }
        self.context.pc(self.origin);
        Ok(())
    }

    fn handle_string(&mut self, s: &str, terminator: Option<u8>) -> Result<(), Error> {
        let zx_safe = ZXAscii::zx_safe(s);
        let ascii_string = match zx_safe.as_ascii_str() {
            Ok(a) => a,
            Err(e) => return Err(self.context.error_text(ErrorType::NonAscii, &e.to_string()))
        };
        self.emit(ascii_string.as_bytes())?;
        if terminator.is_some() {
            self.emit_byte(terminator.unwrap())?;
        }
        Ok(())
    }

    fn handle_bytes(&mut self, terminator: Option<u8>) -> Result<(), Error> {
        let mut expect_comma = false;
        while !&self.tokens.is_empty() {
            if expect_comma {
                self.expect_token(Delimiter(Comma))?
            } else {
                let t = self.take_token()?;
                if let StringLiteral(s) = t {
                    self.tokens.pop();
                    self.handle_string(&s, terminator)?;
                    continue;
                } else {}
                self.tokens.push(t);
                match self.expr.parse(&mut self.context, &mut self.tokens, 0, 1, false) {
                    Ok(Some(n)) => {
                        if !(0..256).contains(&n) {
                            self.warn(ErrorType::IntegerOutOfRange);
                        }
                        self.emit(&[n as u8])?
                    }
                    Ok(None) => if let StringLiteral(s) = self.take_token()? {
                        self.handle_string(&s, terminator)?
                    } else {
                        return Err(self.context.error(ErrorType::SyntaxError));
                    }
                    Err(e) => {
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
                match self.expr.parse(&mut self.context, &mut self.tokens, 0, 2, false) {
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
            (Opt(OptionType::MaxCodeSize), Token::Number(n)) => self.max_code_size(n as usize),
            (_, _) => return Err(self.context.error(ErrorType::InvalidOption))
        };
        Ok(())
    }

    fn locate_file(&mut self, file_name: &str) -> Result<String, Error> {
        let src = self.context.current_file_name().to_string();
        let path = Path::new(&src).parent().unwrap_or(Path::new("."));
        let mut dirs = self.include_dirs.clone();
        if !dirs.contains(&path.to_str().unwrap().to_string()) {
            dirs.insert(0, path.to_str().unwrap().to_string());
        }
        if !dirs.contains(&String::from(".")) {
            dirs.insert(0, String::from("."));
        }
        dirs.reverse();
        while let Some(s) = dirs.pop() {
            let path = Path::new(&s).join(file_name);
            if path.exists() {
                return Ok(path.to_str().unwrap_or("").to_string());
            }
        }
        Err(self.context.error_text(ErrorType::FileNotFound, &file_name))
    }

    fn include_source_file(&mut self) -> Result<(), Error> {
        let file_name = match self.take_token()? {
            StringLiteral(s) => s,
            ConstLabel(l) => l,
            _ => return Err(self.context.error(ErrorType::FileNotFound))
        };
        let file_path = self.locate_file(&file_name)?;
        if self.context.is_included(&file_name) {
            return Err(self.context.error(ErrorType::MultipleIncludes));
        }
        self.info(&format!("Including file from {}", file_path));
        self.first_pass(&file_path)
    }

    fn write_message(&mut self) -> Result<(), Error> {
        if let StringLiteral(s) = self.take_token()? {
            dark_yellow_ln!("{}", s);
        }
        Ok(())
    }

    fn include_binary(&mut self) -> Result<(), Error> {
        let file_name = match self.take_token()? {
            StringLiteral(s) => s,
            ConstLabel(l) => l,
            _ => return Err(self.context.error(ErrorType::FileNotFound))
        };
        let file_path = self.locate_file(&file_name)?;
        self.info(format!("Including binary file from {}", file_path).as_str());
        let mut b: Vec<u8> = vec![];
        let mut f = File::open(&file_path)?;
        let r = f.read_to_end(b.as_mut())? as isize;
        self.context.result(self.bank.append(&mut b))?;
        self.context.add_size_of(r);
        let pc = self.context.offset_pc(r);
        self.context.pc(pc);
        Ok(())
    }

    fn process_if(&mut self) -> Result<(), Error> {
        let label_value: isize;
        let const_value: isize;
        if let ConstLabel(l) = self.take_token()? {
            label_value = match self.context.get_constant(&l) {
                None => return Err(self.context.error(ErrorType::LabelNotFound)),
                Some(n) => n,
            };
        } else {
            return Err(self.context.error(ErrorType::BadConstant));
        }
        self.expect_token(Operator(Equals))?;
        const_value = self.expect_word(-1)?;
        let if_true = label_value == const_value;
        match self.if_level.last() {
            Some(Else(false)) |
            Some(If(false)) |
            Some(SkipEnd) => self.if_level.push(SkipEnd),
            _ => self.if_level.push(IfBlock::If(if_true))
        }

        Ok(())
    }

    fn process_if_def(&mut self) -> Result<(), Error> {
        if let ConstLabel(l) = self.take_token()? {
            let exists = self.context.is_constant_defined(&l);
            match self.if_level.last() {
                Some(Else(false)) |
                Some(If(false)) |
                Some(SkipEnd) => self.if_level.push(SkipEnd),
                _ => self.if_level.push(IfBlock::If(exists))
            }
            Ok(())
        } else {
            Err(self.context.error(ErrorType::BadConstant))
        }
    }

    fn process_endif(&mut self) -> Result<(), Error> {
        if self.if_level.len() == 0 {
            Err(self.context.error(ErrorType::EndIfWithoutIf))
        } else {
            self.if_level.pop();
            Ok(())
        }
    }

    fn process_else(&mut self) -> Result<(), Error> {
        if self.if_level.len() == 0 {
            Err(self.context.error(ErrorType::ElseWithoutIf))
        } else {
            if let Some(If(t)) = self.if_level.pop() {
                self.if_level.push(Else(!t));
            }
            Ok(())
        }
    }

    fn process_global(&mut self) -> Result<(), Error> {
        //unimplemented!()
        Ok(())
    }

    fn process_directive(&mut self, directive: Directive) -> Result<(), Error> {
        match directive {
            Directive::Org => self.set_origin(),
            Directive::Include => self.include_source_file(),
            Directive::Message => self.write_message(),
            Directive::Byte => self.handle_bytes(None),
            Directive::Word => self.handle_words(),
            Directive::Opt => self.set_option(),
            Directive::Binary => self.include_binary(),
            Directive::Block => self.handle_block(),
            Directive::Macro => self.macros.begin_collect(&mut self.context, &mut self.tokens),
            Directive::StringZero => self.handle_bytes(Some(0)),
            Directive::End => {
                if self.macros.collecting() {
                    self.macros.end_collect(&mut self.context)
                } else {
                    return Err(self.context.error(ErrorType::DanglingEnd));
                }
            }
            Directive::Hex => self.handle_hex(),
            Directive::If => self.process_if(),
            Directive::IfDef => self.process_if_def(),
            Directive::Else => self.process_else(),
            Directive::EndIf => self.process_endif(),
            Directive::Global => self.process_global(),
        }
    }
}