use std::collections::HashMap;

use crate::assembler::assembler_context_impl::AssemblerContext;
use crate::assembler::Error;
use crate::assembler::error_impl::ErrorType;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::{Del, Token};
use crate::assembler::tokens::Token::{ConstLabel, Delimiter, MacroParam};

#[derive(Debug, Default)]
pub struct Macro {
    params: Vec<String>,
    tokens: Vec<Vec<Token>>,
}

#[derive(Debug, Default)]
pub struct MacroExpansion {
    params: HashMap<String, Vec<Token>>,
    tokens: Vec<Vec<Token>>,
}

pub struct MacroHandler {
    collecting: bool,
    is_expanding: bool,
    collecting_name: String,
    macros: HashMap<String, Macro>,
    expanding: MacroExpansion,
}

impl MacroHandler {
    pub fn new() -> MacroHandler {
        MacroHandler {
            collecting: false,
            is_expanding: false,
            collecting_name: String::default(),
            macros: HashMap::default(),
            expanding: Default::default(),
        }
    }

    pub fn collecting(&self) -> bool {
        self.collecting
    }

    pub fn expanding(&self) -> bool { self.is_expanding }

    pub fn macro_defined(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    fn parse_params(&mut self, context: &mut AssemblerContext, name: &str, tokens: &mut Vec<Token>) -> Result<(), Error> {
        let mac = &self.macros[name];
        let mut param_count = 0;
        let mut param_name = if mac.params.len() > 0 { &mac.params[param_count] } else { "" };
        let mut expr = vec![];
        while let Some(t) = tokens.pop() {
            if t == Delimiter(Del::Comma) {
                self.expanding.params.insert(param_name.to_string(), expr.clone());
                expr.clear();
                param_count += 1;
                param_name = &mac.params[param_count];
            } else if t.is_expression() {
                expr.push(t);
            } else {
                return Err(context.error(ErrorType::BadExpression));
            }
        }
        param_count += 1;
        if mac.params.len() > 0 && param_count != mac.params.len() {
            return Err(context.error(ErrorType::MacroParamCount));
        }
        self.expanding.params.insert(param_name.to_string(), expr.clone());
        Ok(())
    }

    fn unique_label(name: &str) -> String {
        format!("{}_{:x}", name, rand::random::<u64>()).to_string()
    }

    pub fn begin_expand(&mut self, context: &mut AssemblerContext, name: &str, tokens: &mut Vec<Token>) -> Result<(), Error> {
        self.expanding = Default::default();
        self.parse_params(context, name, tokens)?;
        let mut uses_label = false;
        let mac = &self.macros[name];
        let mut sub = mac.tokens.clone();
        while let Some(mut line) = sub.pop() {
            let mut first_token = true;
            let mut new_line: Vec<Token> = vec![];
            while let Some(tok) = line.pop() {
                match (first_token, &tok) {
                    (true, ConstLabel(label)) =>
                        if label.starts_with(".") {
                            uses_label = true;
                            new_line.push(tok);
                        } else {
                            return Err(context.error(ErrorType::MacroLabel));
                        }
                    (false, MacroParam(name)) => new_line.append(&mut self.expanding.params[&*name].clone()),
                    _ => new_line.push(tok)
                }
                first_token = false;
            }
            self.expanding.tokens.push(new_line);
        }
        if uses_label {
            self.expanding.tokens.insert(0, vec![ConstLabel(Self::unique_label(name).to_string())])
        }
        self.is_expanding = true;
        Ok(())
    }

    pub fn expand(&mut self) -> Option<Vec<Token>> {
        let token = self.expanding.tokens.pop();
        if token.is_none() {
            self.is_expanding = false;
        }
        token
    }

    pub fn begin_collect(&mut self, context: &mut AssemblerContext, tokens: &mut Vec<Token>) -> Result<(), Error> {
        if self.collecting {
            return Err(context.error(ErrorType::NestedMacro));
        }
        let name = match tokens.pop() {
            Some(Token::ConstLabel(l)) => if self.macros.contains_key(&l) {
                return Err(context.error(ErrorType::BadMacroName));
            } else { l },
            _ => return Err(context.error(ErrorType::BadMacroName)) // invalid macro name
        };
        self.collecting_name = name;
        let mut params = vec![];
        let mut expect_comma = false;
        while let Some(t) = tokens.pop() {
            match (expect_comma, t) {
                (false, Token::ConstLabel(l)) => params.push(l),
                (false, Token::Delimiter(Del::Comma)) => return Err(context.error(ErrorType::CommaExpected)),
                (true, Token::Delimiter(Del::Comma)) => {}
                _ => return Err(context.error(ErrorType::BadMacroParam))
            }
            expect_comma = !expect_comma;
        }
        self.collecting = true;
        self.macros.insert(self.collecting_name.clone(), Macro { params, tokens: vec![] });
        Ok(())
    }

    pub fn collect(&mut self, context: &mut AssemblerContext, tokens: &mut Vec<Token>) -> Result<(), Error> {
        if !self.collecting {
            return Err(context.error(ErrorType::NestedMacro));
        }
        let mut tok = vec![];
        let m = self.macros.get_mut(&self.collecting_name).unwrap();
        while let Some(t) = tokens.pop() {
            match &t {
                Token::ConstLabel(l) => {
                    if m.params.contains(l) {
                        tok.push(MacroParam(l.to_string()))
                    } else {
                        tok.push(t)
                    }
                }
                _ => tok.push(t)
            }
        }
        m.tokens.push(tok);

        context.next_line();
        Ok(())
    }


    pub fn end_collect(&mut self, context: &mut AssemblerContext) -> Result<(), Error> {
        if !self.collecting {
            return Err(context.error(ErrorType::DanglingEnd));
        }
        self.collecting_name.clear();
        self.collecting = false;
        Ok(())
    }

//    pub fn dump(&mut self) {
//        println!("-=[ Macros ]=-");
//        for (key, val) in &self.macros {
//            println!("{}({:?})", key, val.params);
//            for l in &val.tokens {
//                let mut ll = l.clone();
//                ll.reverse();
//                println!("\t{:?}", ll);
//            }
//        }
//    }
}