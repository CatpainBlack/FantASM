use crate::assembler::{Assembler, Error};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::tokens::Directive::{End, EndStruct};
use crate::assembler::tokens::Token;
use crate::assembler::tokens::Token::ConstLabel;

pub trait StructHandler {
    fn begin_process_struct(&mut self) -> Result<(), Error>;
    fn end_process_struct(&mut self) -> Result<(), Error>;
    fn process_struct(&mut self) -> Result<(), Error>;
}

impl StructHandler for Assembler {
    fn begin_process_struct(&mut self) -> Result<(), Error> {
        if let ConstLabel(name) = self.take_token()? {
            self.collect_struct = Some((name, 0));
            Ok(())
        } else {
            Err(self.context.error(ErrorType::StructBadName))
        }
    }

    fn end_process_struct(&mut self) -> Result<(), Error> {
        if self.collect_struct.is_some() {
            self.take_token()?;
            self.collect_struct = None;
            Ok(())
        } else {
            Err(self.context.error(ErrorType::StructBadEnd))
        }
    }

    fn process_struct(&mut self) -> Result<(), Error> {
        if self.next_token_is(&Token::Directive(EndStruct)) || self.next_token_is(&Token::Directive(End)) {
            self.end_process_struct()
        } else if let ConstLabel(member) = self.take_token()? {
            let split = member.split(".").collect::<Vec<&str>>();
            let (name, mut val) = self.collect_struct.clone().unwrap();
            let label = format!("{}.{}", name, split[0]);
            //println!("{}={}", label, val);
            self.context.add_constant(label, val)?;
            let suffix = if split.len() == 2 { split[1] } else { "b" };
            match suffix.to_lowercase().as_str() {
                "b" => val += 1,
                "w" => val += 2,
                _ => return Err(self.context.error(ErrorType::StructMemberSize))
            }
            self.collect_struct = Some((name, val));
            Ok(())
        } else {
            Err(self.context.error(ErrorType::StructMemberName))
        }
    }
}