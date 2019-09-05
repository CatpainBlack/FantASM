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

use std::ops::{Index, IndexMut};

use crate::assembler::tokens::{RegPair, Token};
use crate::assembler::tokens::Token::{IndexIndirect, RegisterIX, RegisterIY, RegisterPair};

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