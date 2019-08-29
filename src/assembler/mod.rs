use crate::assembler::expression_impl::ExpressionParser;
use crate::assembler::tokens::Token;
use crate::assembler::assembler_context_impl::AssemblerContext;

mod token_reader_impl;
mod error_impl;
mod tokens;
mod token_traits;
mod number_parser;
mod instruction_encoder;
mod assembler_impl;
mod reg_pair;
mod directive_impl;
mod expression_impl;
mod assembler_context_impl;

//#[derive(Debug)]
struct TokenReader<R> {
	reader: R,
	operators: String,
	delimiters: String,
	line_number: isize,
	words: Vec<String>,
	token_string: String,
	tokens: Vec<Token>,
	preceding_token: Token,
}

#[derive(Debug)]
pub struct ForwardReference {
	is_expression: bool,
	pc: isize,
	label: String,
	expression: Vec<Token>,
	is_relative: bool,
	byte_count: isize,
	line_no: isize,
	file_name: String,
}

pub struct Assembler {
	context: AssemblerContext,
	tokens: Vec<Token>,
	origin: isize,
	bytes: Vec<u8>,
	console_output: bool,
	total_lines: isize,
	expr: ExpressionParser,
	z80n_enabled: bool,
	cspect_enabled: bool,
}

#[derive(Debug)]
pub enum ErrorLevel {
	Fatal,
}

pub struct Error {
	pub line_no: isize,
	pub message: String,
	pub level: ErrorLevel,
	pub file_name: String,
}