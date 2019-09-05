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

use crate::assembler::error_impl::ErrorType;
use crate::assembler::tokens::RegPair::{Af, Bc, De, Hl, Ix, Iy, Sp};
use crate::assembler::tokens::RegPair;

pub trait RegPairValue {
    fn rp1(&self) -> Result<u8, String>;
    fn rp2(&self) -> Result<u8, String>;
    fn nrp(&self) -> Result<u8, String>;
}

impl RegPairValue for RegPair {
    fn rp1(&self) -> Result<u8, String> {
        match self {
            Bc => Ok(0),
            De => Ok(1),
            Hl | Ix | Iy => Ok(2),
            Sp => Ok(3),
            _ => Err(ErrorType::InvalidRegisterPair.to_string())
        }
    }

    fn rp2(&self) -> Result<u8, String> {
        match self {
            Bc => Ok(0),
            De => Ok(1),
            Hl | Ix | Iy => Ok(2),
            Af => Ok(3),
            _ => Err(ErrorType::InvalidRegisterPair.to_string())
        }
    }

    fn nrp(&self) -> Result<u8, String> {
        match self {
            Hl => Ok(0),
            De => Ok(1),
            Bc => Ok(2),
            _ => Err(ErrorType::InvalidRegisterPair.to_string())
        }
    }
}

pub trait HighLow {
    fn lo(&self) -> u8;
    fn hi(&self) -> u8;
}

impl HighLow for u16 {
    fn lo(&self) -> u8 {
        return (self & 0xFF) as u8;
    }
    fn hi(&self) -> u8 {
        (self >> 8u16 & 0xff) as u8
    }
}

impl HighLow for usize {
    fn lo(&self) -> u8 {
        (self & 0xFF) as u8
    }
    fn hi(&self) -> u8 {
        (self >> 8u16 & 0xff) as u8
    }
}

impl HighLow for isize {
    fn lo(&self) -> u8 {
        (self & 0xFF) as u8
    }
    fn hi(&self) -> u8 {
        (self >> 8u16 & 0xff) as u8
    }
}