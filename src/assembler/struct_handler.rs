use std::collections::HashMap;

use crate::assembler::Assembler;
use crate::assembler::constant::Constant;
use crate::assembler::error_type::ErrorType;
use crate::assembler::label::Label;
use crate::assembler::sizeof::SizeOfHandler;
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::Directive::{End, EndStruct};
use crate::assembler::tokens::Token;
use crate::assembler::tokens::Token::{ConstLabel, Delimiter};
use crate::assembler::error::Error;

pub trait StructHandler {
    fn begin_process_struct(&mut self) -> Result<(), Error>;
    fn end_process_struct(&mut self) -> Result<(), Error>;
    fn add_struct_member(&mut self, name: &str, member: &str, size: isize) -> Result<(), Error>;
    fn process_struct(&mut self) -> Result<(), Error>;
    fn is_struct(&self, name: &str) -> bool;
    fn emit_struct(&mut self, name: &str) -> Result<(), Error>;
}

impl StructHandler for Assembler {
    fn begin_process_struct(&mut self) -> Result<(), Error> {
        if let ConstLabel(name) = self.take_token()? {
            if self.is_struct(&name) {
                return Err(self.context.error(ErrorType::StructExists));
            }
            self.context.struct_defs.insert(name.to_string(), HashMap::new());
            self.collect_struct = Some((name, 0));
            Ok(())
        } else {
            Err(self.context.error(ErrorType::StructBadName))
        }
    }

    fn end_process_struct(&mut self) -> Result<(), Error> {
        if self.collect_struct.is_some() {
            let (n, o) = self.collect_struct.clone().unwrap();
            self.context.add_size_of_struct(&n, o);
            self.take_token()?;
            self.collect_struct = None;
            Ok(())
        } else {
            Err(self.context.error(ErrorType::StructBadEnd))
        }
    }

    fn add_struct_member(&mut self, name: &str, member: &str, size: isize) -> Result<(), Error> {
        if let Some(s) = self.context.struct_defs.get_mut(name) {
            s.insert(member.to_string(), size);
        }
        Ok(())
    }

    fn process_struct(&mut self) -> Result<(), Error> {
        if self.next_token_is(&Token::Directive(EndStruct)) || self.next_token_is(&Token::Directive(End)) {
            self.end_process_struct()
        } else if let ConstLabel(member) = self.take_token()? {
            let split = member.split(".").collect::<Vec<&str>>();
            let (name, mut val) = self.collect_struct.clone().unwrap();
            let label = format!("{}.{}", name, split[0]);
            self.context.add_constant(label, val)?;
            let suffix = if split.len() == 2 { split[1] } else { "b" };
            let size = match suffix.to_lowercase().as_str() {
                "b" => 1,
                "w" => 2,
                _ => return Err(self.context.error(ErrorType::StructMemberSize))
            };
            val += size;
            self.add_struct_member(&name, split[0], size)?;
            self.collect_struct = Some((name, val));
            Ok(())
        } else {
            Err(self.context.error(ErrorType::StructMemberName))
        }
    }

    fn is_struct(&self, name: &str) -> bool {
        self.context.struct_defs.contains_key(name)
    }

    fn emit_struct(&mut self, name: &str) -> Result<(), Error> {
        if let Some(def) = self.context.struct_defs.get_mut(name) {
            let mut c = 0;
            let len = def.len() - 1;
            for (k, v) in def.clone() {
                self.context.add_label(format!(".{}", k), false)?;
                if let Ok(Some(n)) = self.expr.parse(&mut self.context, &mut self.tokens, 0, 0, false) {
                    match v {
                        1 => self.emit_byte(n as u8)?,
                        2 => self.emit_word(n)?,
                        _ => return Err(self.context.error(ErrorType::StructMemberSize))
                    };
                    c += 1;
                } else {
                    return Err(self.context.error(ErrorType::IntegerOutOfRange));
                }
                if c <= len {
                    self.expect_token(Delimiter(Comma))?;
                }
            }
        }
        Ok(())
    }
}