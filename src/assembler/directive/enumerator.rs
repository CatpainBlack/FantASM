use crate::assembler::Assembler;
use crate::assembler::collector::Collector;
use crate::assembler::constant::Constant;
use crate::assembler::error::Error;
use crate::assembler::error_type::ErrorType;
use crate::assembler::get_token::GetToken;
use crate::assembler::tokens::{Directive, Token};
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::Op::Equals;
use crate::assembler::tokens::Token::ConstLabel;

pub trait Enumerator {
    fn begin_process_enum(&mut self) -> Result<(), Error>;
    fn end_process_enum(&mut self) -> Result<(), Error>;
    fn process_enum(&mut self) -> Result<(), Error>;
}

impl Enumerator for Assembler {
    fn begin_process_enum(&mut self) -> Result<(), Error> {
        if let ConstLabel(name) = self.take_token()? {
            let count = self.optional_parameter(None)?.unwrap_or(0);
            let step = self.optional_parameter(Some(&Token::Delimiter(Comma)))?.unwrap_or(1);
            if step == 0 {
                return Err(self.context.error(ErrorType::EnumStepValue));
            }
            self.collect_enum = Some((name, count, step));
        } else {
            return Err(self.context.error(ErrorType::EnumBadName));
        }
        Ok(())
    }

    fn end_process_enum(&mut self) -> Result<(), Error> {
        if self.collect_enum.is_some() {
            self.take_token()?;
            self.collect_enum = None;
            Ok(())
        } else {
            Err(self.context.error(ErrorType::EnumBadEnd))
        }
    }

    fn process_enum(&mut self) -> Result<(), Error> {
        if self.next_token_is(&Token::Directive(Directive::EndEnum)) || self.next_token_is(&Token::Directive(Directive::End)) {
            self.end_process_enum()
        } else if let ConstLabel(name) = self.take_token()? {
            let (e, mut v, step) = self.collect_enum.clone().unwrap();
            v = self.optional_parameter(Some(&Token::Operator(Equals)))?.unwrap_or(v);
            let label = format!("{}.{}", e, name);
            self.context.add_constant(label, v)?;
            v += step;
            self.collect_enum = Some((e, v, step));
            Ok(())
        } else {
            Err(self.context.error(ErrorType::EnumMemberName))
        }
    }
}