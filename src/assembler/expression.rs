extern crate asciimath;

use crate::assembler::assembler_context::AssemblerContext;
use crate::assembler::constant::Constant;
use crate::assembler::error_type::ErrorType;
use crate::assembler::ForwardReference;
use crate::assembler::label::Label;
use crate::assembler::sizeof::SizeOfHandler;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::{Op, Token};
use crate::assembler::tokens::Functions::SizeOf;
use crate::assembler::tokens::Token::{ConstLabel, Function, Number, Operator};

use self::asciimath::{eval, scope};

pub struct ExpressionParser {}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser {}
    }

    pub fn get_expression(&mut self, context: &mut AssemblerContext, tokens: &mut Vec<Token>) -> (bool, Vec<Token>) {
        let mut expr = vec![];
        let mut has_forward_ref = false;
        while tokens.last().unwrap_or(&Token::None).is_expression() {
            if let Some(mut t) = tokens.pop() {
                if t == Operator(Op::AsmPc) {
                    t = Number(context.asm_pc())
                }
                expr.push(t.clone());
            }
            let last = expr.last().unwrap_or(&Token::None).clone();
            if let ConstLabel(l) = last {
                if l.to_lowercase().eq(&"asmpc".to_string()) {
                    expr.pop();
                    expr.push(Number(context.asm_pc()));
                } else if !context.is_constant_defined(&l) && !context.is_label_defined(&l) {
                    has_forward_ref = true;
                }
            } else if let Function(SizeOf(label)) = last {
                if !context.is_label_defined(&label) {
                    has_forward_ref = true;
                }
            }
        }
        (has_forward_ref, expr)
    }

    pub fn eval(&self, context: &mut AssemblerContext, expr: &mut Vec<Token>) -> Result<isize, ErrorType> {
        let mut strings = vec![];
        for token in expr {
            match &token {
                Function(SizeOf(label)) => if let Some(size) = context.get_size_of(label) {
                    strings.push(format!("{}", size));
                } else {
                    return Err(ErrorType::UnknownSizeOf);
                }
                ConstLabel(l) => {
                    if let Some(n) = context.get_constant(l) {
                        strings.push(format!("{}", n));
                    } else if let Some(n) = context.get_label(l) {
                        strings.push(format!("{}", n));
                    } else {
                        return Err(ErrorType::BadConstant);
                    }
                }
                Number(_) | Operator(_) => strings.push(token.to_string()),
                _ => return Err(ErrorType::BadExpression)
            }
        }
        let string_expr = strings.join("");
        match eval(&string_expr, &scope! {}) {
            Ok(r) => {
                Ok(r as isize)
            }
            Err(_e) => return Err(ErrorType::BadExpression),
        }
    }

    pub fn parse(&mut self, context: &mut AssemblerContext, tokens: &mut Vec<Token>, offset: isize, count: isize, is_relative: bool) -> Result<Option<isize>, ErrorType> {
        let (has_forward_ref, mut expr) = self.get_expression(context, tokens);
        if has_forward_ref && count < 0 {
            return Err(ErrorType::BadConstant);
        }
        if has_forward_ref {
            let fw = ForwardReference {
                is_expression: true,
                pc: context.offset_pc(offset),
                label: context.label_context.clone(),
                expression: expr,
                is_relative,
                byte_count: count,
                line_no: context.current_line_number(),
                file_name: context.current_file_name().to_string(),
            };
            context.add_forward_ref(fw);
            return Ok(Some(0));
        }

        match self.eval(context, expr.as_mut()) {
            Ok(n) => Ok(Some(n)),
            Err(e) => return Err(e),
        }
    }
}
