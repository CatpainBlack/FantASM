use crate::assembler::error_impl::ErrorType;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::Token;
use crate::assembler::tokens::Op::{Add, Ampersand, Div, Mul, Pipe, Shl, Shr, Sub};
use crate::assembler::tokens::Token::{ConstLabel, Number, Operator};
use crate::assembler::ForwardReference;
use crate::assembler::assembler_context_impl::AssemblerContext;

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

	fn get_expression(&mut self, context: &mut AssemblerContext, tokens: &mut Vec<Token>) -> (bool, Vec<Token>) {
		let mut expr = vec![];
		let mut has_forward_ref = false;
		while tokens.last().unwrap_or(&Token::None).is_expression() {
			expr.push(tokens.pop().unwrap());
			if let Some(ConstLabel(l)) = expr.last() {
				if !context.is_constant_defined(l) && !context.is_label_defined(l) {
					has_forward_ref = true;
				}
			}
		}
		(has_forward_ref, expr)
	}

	pub fn eval(&mut self, context: &mut AssemblerContext, expr: &mut Vec<Token>) -> Result<isize, ErrorType> {
		self.op = Operator(Add);
		self.accumulator = 0;
		for token in expr {
			match token {
				ConstLabel(l) => {
					if let Some(n) = context.get_constant(l) {
						self.accumulate(n.clone())?;
					} else if let Some(n) = context.get_label(l) {
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

	pub fn parse(&mut self, context: &mut AssemblerContext, tokens: &mut Vec<Token>, offset: isize, count: isize) -> Result<Option<isize>, ErrorType> {
		let (has_forward_ref, mut expr) = self.get_expression(context, tokens);
		if has_forward_ref && count < 0 {
			return Err(ErrorType::BadConstant);
		}
		if has_forward_ref {
			let fw = ForwardReference {
				is_expression: true,
				pc: context.offset_pc(offset),
				label: "".to_string(),
				expression: expr,
				is_relative: false,
				byte_count: count,
				line_no: context.current_line_number(),
				file_name: context.current_file_name().to_string(),
			};
			context.add_forward_ref(fw);
			return Ok(Some(0));
		}

		match self.eval(context, expr.as_mut()) {
			Ok(_) => Ok(Some(self.accumulator)),
			Err(e) => return Err(e),
		}
	}
}
