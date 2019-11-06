use crate::assembler::assembler_context::AssemblerContext;
use crate::assembler::bank::Bank;
use crate::assembler::directive::macros::MacroHandler;
use crate::assembler::expression::ExpressionParser;
use crate::assembler::tokens::Token;

pub(super) mod assembler_options;
pub(super) mod error;
mod token_reader;
mod error_type;
mod tokens;
mod token_traits;
mod number_parser;
mod instruction_encoder;
mod assembler;
mod reg_pair;
mod directive;
mod expression;
mod assembler_context;
mod bank;
mod token_to_string_impl;
mod macros;
mod zx_ascii;
mod collector;
mod label;
mod constant;
mod emitter;
mod get_token;

struct TokenReader<R> {
    reader: R,
    operators: String,
    delimiters: String,
    line_number: isize,
    words: Vec<String>,
    token_string: String,
    tokens: Vec<Token>,
    preceding_token: Token,
    file_name: String,
    whitespace_at_start: bool,
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

#[derive(Debug)]
pub enum IfBlock {
    None,
    If(bool),
    Else(bool),
    SkipEnd,
}

pub struct Assembler {
    context: AssemblerContext,
    macros: MacroHandler,
    tokens: Vec<Token>,
    origin: isize,
    bank: Bank,
    console_output: bool,
    total_lines: isize,
    expr: ExpressionParser,
    z80n_enabled: bool,
    c_spect_enabled: bool,
    debug: bool,
    collect_macro: bool,
    collect_enum: Option<(String, isize, isize)>,
    collect_struct: Option<(String, isize)>,
    warnings: Vec<String>,
    include_dirs: Vec<String>,
    labels_file: String,
    if_level: Vec<IfBlock>,
    defines: Vec<(String)>,
}



