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

use std::io::BufRead;

use crate::assembler::{Error, TokenReader};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::{Cnd, Op, Reg, RegPair, RegPairInd, Token};
use crate::assembler::tokens::Op::{LParens, RParens};
use crate::assembler::tokens::Token::{Condition, ConstLabel, IndexIndirect, IndirectExpression, Number, Operator, Register, RegisterIndirect, RegisterPair};

impl<R> TokenReader<R> where R: BufRead {
    pub fn new(reader: R) -> TokenReader<R> {
        TokenReader {
            reader,
            operators: String::new(),
            delimiters: String::new(),
            line_number: 0,
            token_string: String::new(),
            words: vec![],
            tokens: vec![],
            preceding_token: Token::EndOfFile,
            file_name: String::default(),
        }
    }

    pub fn delimiters(&mut self, del: &str) -> &mut TokenReader<R> {
        self.delimiters = del.to_string();
        self
    }

    pub fn operators(&mut self, ops: &str) -> &mut TokenReader<R> {
        self.operators = ops.to_string();
        self
    }

    pub fn file_name(&mut self, file_name: &str) -> &mut TokenReader<R> {
        self.file_name = file_name.to_string();
        self
    }

    fn store_token_string(&mut self) {
        if !self.token_string.is_empty() {
            self.words.push(self.token_string.replace("\\", ""));
            self.token_string.clear();
        }
    }

    fn split_line(&mut self, line: &String) {
        let mut in_quotes = false;
        self.words = vec![];

        for c in line.chars() {
            if in_quotes {
                self.token_string.push(c);
                if (c == '\"' || c == '\'') && !self.token_string.ends_with("\\\"") {
                    in_quotes = false;
                    self.store_token_string();
                }
                continue;
            }

            match c {
                // if we hit a comment
                ';' => break,
                // if we are at the start of a string literal
                '\"' | '\'' => {
                    if !self.token_string.to_lowercase().ends_with("af") {
                        in_quotes = true;
                        self.store_token_string();
                    }
                    self.token_string.push(c);
                    continue;
                }
                // check for operators that are double characters
                s @ '<' | s @ '>' => if self.words.last() == Some(&s.to_string()) {
                    self.words.pop();
                    self.words.push(format!("{}{}", s, s));
                    self.token_string.clear();
                    continue;
                }
                _ => {}
            }
            if c == ':' && self.words.len() > 0 {
                self.store_token_string();
                continue;
            }
            let is_operator = self.operators.find(c).is_some();
            let is_delimiter = self.delimiters.find(c).is_some();
            let is_whitespace = c.is_whitespace();
            let brk = in_quotes || is_whitespace || is_delimiter || is_operator;
            if brk {
                self.store_token_string();
                if !c.is_whitespace() {
                    self.words.push(c.to_string());
                }
            } else if !c.is_whitespace() {
                self.token_string.push(c);
            }
        }
        self.store_token_string();
        self.words.reverse();
//        println!("{:?}", self.words);
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.words.is_empty() {
            return None;
        }
        let w = self.words.pop().unwrap_or(String::new());
        let mut tok = Token::from_string(w);
        if self.preceding_token.can_be_conditional() && tok == Register(Reg::C) {
            tok = Condition(Cnd::C)
        }
        self.preceding_token = tok.clone();
        Some(tok.to_owned())
    }

    fn handle_index_indirect(&mut self, tokens: &mut Vec<Token>, rp: RegPair) -> Result<Option<Token>, Error> {
        let mut index = 0;
        if let Some(Operator(Op::Add)) = tokens.pop()
        {
            if let Some(Number(n)) = tokens.pop() {
                if n > 255 {
                    return Err(Error::fatal("Integer out of range", self.line_number, &self.file_name));
                }
                index = n as u8;
            }
        }
        if tokens.is_empty() {
            return Ok(Some(IndexIndirect(rp, index)));
        }
        Ok(None)
    }

    fn handle_parentheses(&mut self, s: usize, e: usize) -> Result<(), Error> {
        if e - s >= 3 {
            let mut expr = self.tokens[s + 1..e - 1].to_vec();
            expr.reverse();
            if let Some(c) = match expr.pop() {
                Some(RegisterPair(RegPair::Ix)) => self.handle_index_indirect(&mut expr, RegPair::Ix)?,
                Some(RegisterPair(RegPair::Iy)) => self.handle_index_indirect(&mut expr, RegPair::Iy)?,
                Some(RegisterPair(RegPair::Bc)) => Some(RegisterIndirect(RegPairInd::Bc)),
                Some(RegisterPair(RegPair::Sp)) => Some(RegisterIndirect(RegPairInd::Sp)),
                Some(RegisterPair(RegPair::De)) => Some(RegisterIndirect(RegPairInd::De)),
                Some(RegisterPair(RegPair::Hl)) => Some(Register(Reg::_HL_)),
                Some(Register(Reg::C)) => if self.preceding_token.can_be_conditional() {
                    Some(Condition(Cnd::C))
                } else {
                    Some(RegisterIndirect(RegPairInd::C))
                },
                Some(Number(n)) => Some(IndirectExpression(vec![Number(n)])),
                Some(ConstLabel(l)) => Some(IndirectExpression(vec![ConstLabel(l)])),
                _ => None
            } {
                if expr.is_empty() {
                    self.tokens.truncate(s);
                    self.tokens.push(c);
                }
            }
        }
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<Vec<Token>, Error> {
        let mut line = String::new();
        let mut parens: Vec<usize> = vec![];
        self.line_number += 1;
        let count = self.reader.read_line(&mut line)?;
        if count <= 0 {
            return Ok(vec![Token::EndOfFile]);
        }
        self.split_line(&line);
        self.tokens.clear();
        let mut pos = 0;
        while let Some(tok) = self.next_token() {
            match tok {
                Operator(LParens) => {
                    parens.push(pos);
                }
                Operator(RParens) => {
                    if let Some(s) = parens.pop() {
                        self.tokens.push(tok.clone());
                        self.handle_parentheses(s, pos + 1)?;
                        continue;
                    } else {
                        return Err(Error::fatal(&ErrorType::UnexpectedClose.to_string(), self.line_number, &self.file_name));
                    }
                }
                _ => {}
            }
            self.tokens.push(tok.clone());
            pos += 1;
        }
        if !parens.is_empty() {
            return Err(Error::fatal(&ErrorType::UnclosedParentheses.to_string(), self.line_number, &self.file_name));
        }
        Ok(self.tokens.to_owned())
    }
}