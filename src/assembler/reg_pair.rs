use crate::assembler::error_impl::ErrorType;
use crate::assembler::tokens::RegPair::{Af, Bc, De, Hl, Ix, Iy, Sp};
use crate::assembler::tokens::RegPair;

pub trait RegPairValue {
    fn rp1(&self) -> Result<u8, String>;
    fn rp2(&self) -> Result<u8, String>;
}

impl RegPairValue for RegPair {
    fn rp1(&self) -> Result<u8, String> {
        return match self {
            Bc => Ok(0),
            De => Ok(1),
            Hl | Ix | Iy => Ok(2),
            Sp => Ok(3),
            _ => Err(ErrorType::InvalidRegisterPair.to_string())
        };
    }

    fn rp2(&self) -> Result<u8, String> {
        return match self {
            Bc => Ok(0),
            De => Ok(1),
            Hl | Ix | Iy => Ok(2),
            Af => Ok(3),
            _ => Err(ErrorType::InvalidRegisterPair.to_string())
        };
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
        return (self >> 8u16 & 0xff) as u8;
    }
}

impl HighLow for usize {
    fn lo(&self) -> u8 {
        return (self & 0xFF) as u8;
    }
    fn hi(&self) -> u8 {
        return (self >> 8u16 & 0xff) as u8;
    }
}

impl HighLow for isize {
    fn lo(&self) -> u8 {
        return (self & 0xFF) as u8;
    }
    fn hi(&self) -> u8 {
        return (self >> 8u16 & 0xff) as u8;
    }
}