use std::ops::Range;

use crate::assembler::Assembler;
use crate::assembler::error::Error;
use crate::assembler::error_type::ErrorType;
use crate::assembler::tokens::Token;

pub trait GetToken {
    fn relative(&mut self) -> Result<u8, Error>;
    fn expect_byte(&mut self, instr_size: isize) -> Result<isize, Error>;
    fn expect_word(&mut self, instr_size: isize) -> Result<isize, Error>;
    fn expect_number_in_range(&mut self, range: Range<isize>, count: isize, error_type: ErrorType, instr_size: isize) -> Result<isize, Error>;
    fn take_token(&mut self) -> Result<Token, Error>;
    fn next_token_is(&mut self, tok: &Token) -> bool;
    fn expect_token(&mut self, tok: Token) -> Result<(), Error>;
}

impl GetToken for Assembler {
    fn relative(&mut self) -> Result<u8, Error> {
        let addr = match self.expr.parse(&mut self.context, &mut self.tokens, 1, 1, true) {
            Ok(Some(n)) => n,
            Ok(None) => 0,
            Err(e) => return Err(self.context.error(e)),
        };
        let pc = (self.context.offset_pc(2)) as isize;
        Ok((addr - pc) as u8)
    }

    fn expect_byte(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..256, 1, ErrorType::ByteTruncated, instr_size)
    }

    fn expect_word(&mut self, instr_size: isize) -> Result<isize, Error> {
        self.expect_number_in_range(0..65536, 2, ErrorType::WordTruncated, instr_size)
    }

    fn expect_number_in_range(&mut self, range: Range<isize>, count: isize, error_type: ErrorType, instr_size: isize) -> Result<isize, Error> {
        match self.expr.parse(&mut self.context, &mut self.tokens, instr_size, count, false) {
            Ok(Some(n)) => {
                if !range.contains(&n) {
                    self.warn(error_type);
                }
                Ok(n)
            }
            Ok(None) => return Err(self.context.error(ErrorType::SyntaxError)),
            Err(e) => return Err(self.context.error(e))
        }
    }

    fn take_token(&mut self) -> Result<Token, Error> {
        if let Some(tok) = self.tokens.pop() {
            return Ok(tok);
        }
        Err(self.context.error(ErrorType::UnexpectedEndOfLine))
    }

    fn next_token_is(&mut self, tok: &Token) -> bool {
        if let Some(t) = self.tokens.last() {
            t == tok
        } else {
            false
        }
    }

    fn expect_token(&mut self, tok: Token) -> Result<(), Error> {
        let t = self.take_token()?;
        if t != tok {
            return Err(self.context.error(ErrorType::SyntaxError));
        }
        Ok(())
    }
}