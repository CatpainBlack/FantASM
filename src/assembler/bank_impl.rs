use std::ops::{IndexMut, Index};
use crate::assembler::tokens::{Token, RegPair};
use crate::assembler::tokens::Token::{RegisterPair, RegisterIX, IndexIndirect, RegisterIY};

pub struct Bank {
    bytes: Vec<u8>
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            bytes: vec![]
        }
    }

    pub fn as_slice(&mut self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn append(&mut self, bytes: &mut Vec<u8>) {
        self.bytes.append(bytes);
    }

    pub fn push(&mut self, b: u8) {
        self.bytes.push(b);
    }

    pub fn emit_prefix(&mut self, t: &Token) -> isize {
        match t {
            RegisterPair(RegPair::Ix) | RegisterIX(_) | IndexIndirect(RegPair::Ix, _) => self.push(0xDD),
            RegisterPair(RegPair::Iy) | RegisterIY(_) | IndexIndirect(RegPair::Iy, _) => self.push(0xFD),
            _ => return 0
        }
        1
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