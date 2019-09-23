use std::ops::{Index, IndexMut};

use crate::assembler::tokens::{RegPair, Token};
use crate::assembler::tokens::Token::{IndexIndirect, RegisterIX, RegisterIY, RegisterPair};
use crate::assembler::error_impl::ErrorType;

pub struct Bank {
    bytes: Vec<u8>,
    max_size: usize,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            bytes: vec![],
            max_size: 65536,
        }
    }

    pub fn max_code_size(&mut self, size: usize) {
        self.max_size = size;
    }

    pub fn as_slice(&mut self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn append(&mut self, bytes: &mut Vec<u8>) -> Result<(), ErrorType> {
        self.bytes.append(bytes);
        if self.bytes.len() > self.max_size {
            return Err(ErrorType::CodeSize);
        }
        Ok(())
    }

    pub fn push(&mut self, b: u8) -> Result<(), ErrorType> {
        self.bytes.push(b);
        if self.bytes.len() > self.max_size {
            return Err(ErrorType::CodeSize);
        }
        Ok(())
    }

    pub fn emit_prefix(&mut self, t: &Token) -> Result<isize, ErrorType> {
        match t {
            RegisterPair(RegPair::Ix) | RegisterIX(_) | IndexIndirect(RegPair::Ix, _) => self.push(0xDD)?,
            RegisterPair(RegPair::Iy) | RegisterIY(_) | IndexIndirect(RegPair::Iy, _) => self.push(0xFD)?,
            _ => return Ok(0)
        }
        Ok(1)
    }
}

impl IndexMut<usize> for Bank {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bytes[index]
    }
}

impl Index<usize> for Bank {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}