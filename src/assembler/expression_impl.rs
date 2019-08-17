use std::collections::HashMap;

use crate::assembler::error_impl::ErrorType;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::Token;
use crate::assembler::tokens::Op::{Add, Ampersand, Div, Mul, Pipe, Shl, Shr, Sub};
use crate::assembler::tokens::Token::{Label, Number, Operator};

pub struct ExpressionParser {
    accumulator: isize,
    op: Token,
}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser {
            accumulator: 0,
            op: Token::Operator(Add),
        }
    }

    fn accumulate(&mut self, number: isize) -> Result<(), ErrorType> {
        match self.op {
            Operator(Add) => self.accumulator += number,
            Operator(Sub) => self.accumulator -= number,
            Operator(Div) => {
                if number == 0 {
                    return Err(ErrorType::DivideByZero);
                }
                self.accumulator /= number
            }
            Operator(Mul) => self.accumulator *= number,
            Operator(Shl) => self.accumulator <<= number,
            Operator(Shr) => self.accumulator >>= number,
            Operator(Ampersand) => self.accumulator &= number,
            Operator(Pipe) => self.accumulator |= number,
            _ => return Err(ErrorType::BadOperator)
        }
        Ok(())
    }

    pub fn parse(&mut self, tokens: &mut Vec<Token>, constants: &mut HashMap<String, isize>, labels: &mut HashMap<String, isize>) -> Result<Option<isize>, ErrorType> {
        let mut count = 0;
        self.op = Operator(Add);
        self.accumulator = 0;
        while let Some(token) = tokens.pop() {
            if !token.is_expression() {
                tokens.push(token.clone());
                break;
            }
            count += 1;
            match token {
                Label(l) => {
                    if let Some(n) = constants.get(l.as_str()) {
                        self.accumulate(n.clone())?;
                    } else if let Some(n) = labels.get(l.as_str()) {
                        self.accumulate(n.clone())?;
                    } else {
                        return Err(ErrorType::BadConstant);
                    }
                }
                Number(n) => self.accumulate(n)?,
                Operator(o) => self.op = Operator(o),
                _ => {
                    break;
                }
            }
        }
        if count > 0 {
            Ok(Some(self.accumulator))
        } else {
            Ok(None)
        }
    }
}
