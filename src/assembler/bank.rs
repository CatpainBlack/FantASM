use std::ops::{IndexMut, Index};

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

//    pub fn as_mut_slice(&mut self) -> &mut [u8] {
//        self.bytes.as_mut_slice()
//    }

    pub fn append(&mut self, bytes: &mut Vec<u8>) {
        self.bytes.append(bytes);
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