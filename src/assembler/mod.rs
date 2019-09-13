/*
Copyright (c) 2019, Guy Black
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

The views and conclusions contained in the software and documentation are those
of the authors and should not be interpreted as representing official policies,
either expressed or implied, of the FantASM project.
*/

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
    macros: MacroHandler,
    tokens: Vec<Token>,
    origin: isize,
    bank: Bank,
    console_output: bool,
    total_lines: isize,
    expr: ExpressionParser,
    z80n_enabled: bool,
    cspect_enabled: bool,
    debug: bool,
    collect_macro: bool,
    warnings: Vec<String>,
    include_dirs: Vec<String>,
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