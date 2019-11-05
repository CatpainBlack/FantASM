use crate::assembler::Assembler;
use crate::assembler::error::Error;
use crate::assembler::tokens::Token;
use crate::assembler::tokens::Token::Number;

pub trait Collector {
    fn optional_parameter(&mut self, preceded_by: Option<&Token>) -> Result<Option<isize>, Error>;
}

impl Collector for Assembler {
    fn optional_parameter(&mut self, preceded_by: Option<&Token>) -> Result<Option<isize>, Error> {
        if preceded_by.is_some() {
            let last = self.tokens.last();
            if last != preceded_by {
                return Ok(None);
            }
            self.tokens.pop();
        }
        if let Ok(Number(n)) = self.take_token() {
            Ok(Some(n))
        } else {
            Ok(None)
        }
    }
}