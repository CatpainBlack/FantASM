use std::collections::HashMap;

use crate::assembler::error_impl::ErrorType;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::Token;
use crate::assembler::tokens::Op::{Add, Ampersand, Div, Mul, Pipe, Shl, Shr, Sub};
use crate::assembler::tokens::Token::{ConstLabel, Number, Operator};
use core::ptr;
use crate::assembler::ForwardReference;

pub struct ExpressionParser {
    accumulator: isize,
    op: Token,
    labels: *mut HashMap<String, isize>,
    constants: *mut HashMap<String, isize>,
    forward_references: *mut Vec<ForwardReference>,
}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser {
            accumulator: 0,
            op: Token::Operator(Add),
            labels: ptr::null_mut(),
            constants: ptr::null_mut(),
            forward_references: ptr::null_mut(),
        }
    }

    pub fn init(
        &mut self,
        labels: *mut HashMap<String, isize>,
        constants: *mut HashMap<String, isize>,
        forward_references: *mut Vec<ForwardReference>)
    {
        self.labels = labels;
        self.constants = constants;
        self.forward_references = forward_references;
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

    fn get_expression(&mut self, tokens: &mut Vec<Token>) -> (bool, Vec<Token>) {
        let mut expr = vec![];
        let mut has_forward_ref = false;
        while tokens.last().unwrap_or(&Token::None).is_expression() {
            expr.push(tokens.pop().unwrap());
            if let Some(ConstLabel(l)) = expr.last() {
                unsafe {
                    if !(*self.constants).contains_key(l) && !(*self.labels).contains_key(l) {
                        has_forward_ref = true;
                    }
                }
            }
        }
        (has_forward_ref, expr)
    }

    pub fn eval(&mut self, expr: &mut Vec<Token>) -> Result<isize, ErrorType> {
        self.op = Operator(Add);
        self.accumulator = 0;
        for token in expr {
            match token {
                ConstLabel(l) => {
                    if let Some(n) = unsafe { (*self.constants).get(l) } {
                        self.accumulate(n.clone())?;
                    } else if let Some(n) = unsafe { (*self.labels).get(l) } {
                        self.accumulate(n.clone())?;
                    } else {
                        return Err(ErrorType::BadConstant);
                    }
                }
                Number(n) => self.accumulate(*n)?,
                Operator(o) => self.op = Operator(o.clone()),
                _ => return Err(ErrorType::BadExpression)
            }
        }
        Ok(self.accumulator)
    }

    pub fn parse(&mut self, tokens: &mut Vec<Token>, pc: isize) -> Result<Option<isize>, ErrorType> {
        let (has_forward_ref, mut expr) = self.get_expression(tokens);

        if has_forward_ref {
            //println!("Expression FW REF:{} {:?}", pc, expr);
            unsafe {
                (*self.forward_references).push(ForwardReference {
                    is_expression: true,
                    pc,
                    label: "".to_string(),
                    expression: expr,
                    swap_bytes: false,
                    relative: false,
                });
            }
            return Ok(Some(0));
        }

        match self.eval(expr.as_mut()) {
            Ok(_) => Ok(Some(self.accumulator)),
            Err(e) => return Err(e),
        }
    }
}
