use crate::assembler::assembler_context_impl::AssemblerContext;
use crate::assembler::bank_impl::Bank;
use crate::assembler::expression_impl::ExpressionParser;
use crate::assembler::macro_impl::MacroHandler;
use crate::assembler::tokens::Token;

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
mod bank_impl;
mod macro_impl;
mod token_to_string_impl;
mod macros;
mod zx_ascii;
mod enum_handler_impl;
mod conditional_impl;
mod collector_impl;

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
    warnings: Vec<String>,
    include_dirs: Vec<String>,
    labels_file: String,
    if_level: Vec<IfBlock>,
    defines: Vec<(String)>,
    next_label_global: bool,

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
