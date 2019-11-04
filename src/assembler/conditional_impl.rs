use crate::assembler::{Assembler, Error, IfBlock};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::IfBlock::{Else, If, SkipEnd};
use crate::assembler::tokens::Op::Equals;
use crate::assembler::tokens::Token::{ConstLabel, Operator};

pub trait Conditional {
    fn process_if(&mut self) -> Result<(), Error>;
    fn process_if_def(&mut self, defined: bool) -> Result<(), Error>;
    fn process_endif(&mut self) -> Result<(), Error>;
    fn process_else(&mut self) -> Result<(), Error>;
}

impl Conditional for Assembler {
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

    fn process_if_def(&mut self, defined: bool) -> Result<(), Error> {
        if let ConstLabel(l) = self.take_token()? {
            let mut exists = self.context.is_constant_defined(&l);
            if !defined {
                exists = !exists;
            }
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
}