use std::str::FromStr;

use regex::Regex;

use crate::assembler::number_parser::NumberParser;
use crate::assembler::tokens::{RegPair, Token};
use crate::assembler::tokens::{Cnd, Del, Directive, Ir, IxU, IyU, Op, OpCode, Reg};
use crate::assembler::tokens::Token::{AddressIndirect, IndexIndirect, Label, Number, Operator, Register, RegisterIndirect, RegisterIR, RegisterIX, RegisterIY, RegisterPair};

pub trait Tokens {
    fn from_string(word: String) -> Token;
    fn is_index_prefix(&self) -> Option<u8>;
    fn is_indirect(&self) -> bool;
    fn is_reg(&self) -> bool;
    fn is_reg_pair(&self) -> bool;
    fn is_special_reg(&self) -> bool;
    fn is_expression(&self) -> bool;
    fn can_be_conditional(&self) -> bool;
    fn number_to_u8(&self) -> Option<u8>;
    fn reg_value(&self) -> Option<u8>;
}

impl Tokens for Token {
    fn from_string(word: String) -> Token {
        let re_label = Regex::new(r"^\.?[a-zA-Z]+[a-zA-Z0-9_]*:?$").unwrap();

        let w = word.to_lowercase();

        // string literals
        if word.starts_with("\"") && word.ends_with("\"") {
            if let Some(s) = word.get(1..word.len() - 1) {
                return Token::StringLiteral(s.to_string());
            }
        }

        // Directives
        if let Ok(d) = Directive::from_str(w.as_str()) {
            return Token::Directive(d);
        }
        //Opcodes
        if let Ok(o) = OpCode::from_str(w.as_str()) {
            return Token::OpCode(o);
        }
        // Numbers
        if let Some(n) = word.to_number() {
            return Token::Number(n as isize);
        }
        //Register pairs
        if let Ok(rp) = RegPair::from_str(w.as_str()) {
            return Token::RegisterPair(rp);
        }
        // Registers
        if let Ok(r) = Reg::from_str(w.as_str()) {
            return Token::Register(r);
        }

        // I/R Register
        if let Ok(r) = Ir::from_str(&w.as_str()) {
            return Token::RegisterIR(r);
        }

        // Delimiters
        if let Ok(d) = Del::from_str(w.as_str()) {
            return Token::Delimiter(d);
        }
        // Operators
        if let Ok(op) = Op::from_str(w.as_str()) {
            return Token::Operator(op);
        }
        // IHx/IXh
        if let Ok(ixu) = IxU::from_str(w.as_str()) {
            return Token::RegisterIX(ixu);
        }
        // IYx/IYh
        if let Ok(iyu) = IyU::from_str(w.as_str()) {
            return Token::RegisterIY(iyu);
        }
        // Conditions
        if let Ok(cnd) = Cnd::from_str(w.as_str()) {
            return Token::Condition(cnd);
        }
        // Label
        if re_label.is_match_at(&word, 0) {
            //println!("Label: {}", word);
            return Token::Label(word);
        }

        return Token::Invalid;
    }

    fn is_index_prefix(&self) -> Option<u8> {
        match self {
            RegisterPair(RegPair::Ix) | RegisterIX(_) => Some(0xDD),
            RegisterPair(RegPair::Iy) | RegisterIY(_) => Some(0xFD),
            _ => None
        }
    }

    fn is_indirect(&self) -> bool {
        match self {
            RegisterIndirect(_) => true,
            AddressIndirect(_) => true,
            IndexIndirect(_, _) => true,
            _ => false
        }
    }

    fn is_reg(&self) -> bool {
        match self {
            Register(_) | RegisterIX(_) | RegisterIY(_) => true,
            _ => false
        }
    }

    fn is_reg_pair(&self) -> bool {
        return match self {
            RegisterPair(_) => true,
            _ => false
        };
    }

    fn is_special_reg(&self) -> bool {
        match self {
            RegisterPair(RegPair::Sp) => true,
            RegisterIR(_) => true,
            _ => false
        }
    }

    fn is_expression(&self) -> bool {
        match self.clone() {
            Number(_) => true,
            Operator(_) => true,
            Label(_) => true,
            _ => false
        }
    }

    fn can_be_conditional(&self) -> bool {
        match self {
            Token::OpCode(OpCode::Jr) => true,
            Token::OpCode(OpCode::Ret) => true,
            Token::OpCode(OpCode::Call) => true,
            Token::OpCode(OpCode::Jp) => true,
            _ => false
        }
    }

    fn number_to_u8(&self) -> Option<u8> {
        match self {
            Number(n) => if (0..256).contains(n) {
                Some(n.clone() as u8)
            } else { None }
            AddressIndirect(n) => if (0..256).contains(n) {
                Some(n.clone() as u8)
            } else { None }
            _ => None
        }
    }

    fn reg_value(&self) -> Option<u8> {
        match self {
            Register(r) => Some(r.clone() as u8),
            RegisterIX(r) => Some(r.clone() as u8),
            RegisterIY(r) => Some(r.clone() as u8),
            RegisterPair(r) => Some(r.clone() as u8),
            _ => None
        }
    }
}
