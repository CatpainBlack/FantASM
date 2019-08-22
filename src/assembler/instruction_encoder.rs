use crate::assembler::{Assembler, Error};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::reg_pair::HighLow;
use crate::assembler::reg_pair::RegPairValue;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::{AluOp, Cnd, Ir, Reg, RegPairInd, RotOp, Token};
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::Reg::_HL_;
use crate::assembler::tokens::RegPair::{_Af, Af, De, Hl, Ix, Iy, Sp};
use crate::assembler::tokens::Token::{AddressIndirect, Condition, Delimiter, IndexIndirect, ConstLabel, Number, Register, RegisterIndirect, RegisterIR, RegisterIX, RegisterIY, RegisterPair, ConstLabelIndirect};
use crate::assembler::error_impl::ErrorType::{InvalidInstruction, ByteTrunctated};

pub(crate) trait InstructionEncoder {
    fn xyz(x: u8, y: u8, z: u8) -> u8;
    fn xpqz(x: u8, p: u8, q: u8, z: u8) -> u8;
    fn alu(op: AluOp, r: u8) -> u8;
    fn rot_encode(op: RotOp, r: u8) -> u8;
    fn alu_imm(op: AluOp) -> u8;
    fn alu_op(&mut self, a: AluOp) -> Result<Vec<u8>, Error>;
    fn alu_op_r(&mut self, a: AluOp, x: u8, q: u8) -> Result<Vec<u8>, Error>;
    fn bit_res_set(&mut self, x: u8) -> Result<Vec<u8>, Error>;

    fn call_jp(&mut self, q: u8, z: u8) -> Result<Vec<u8>, Error>;
    fn jp(&mut self) -> Result<Vec<u8>, Error>;

    fn ex(&mut self) -> Result<Vec<u8>, Error>;
    fn im(&mut self) -> Result<Vec<u8>, Error>;
    fn inc_dec(&mut self, q: u8) -> Result<Vec<u8>, Error>;
    fn io_op(&mut self, y: u8) -> Result<Vec<u8>, Error>;
    fn jr(&mut self) -> Result<Vec<u8>, Error>;
    fn push_pop(&mut self, z: u8) -> Result<Vec<u8>, Error>;
    fn ret(&mut self) -> Result<Vec<u8>, Error>;
    fn rot(&mut self, a: RotOp) -> Result<Vec<u8>, Error>;
    fn rst(&mut self) -> Result<Vec<u8>, Error>;
    fn load(&mut self) -> Result<Vec<u8>, Error>;
    fn load_indirect(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error>;
    fn load_r(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error>;
    fn load_rp(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error>;
    fn load_special(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error>;
    fn mul(&mut self) -> Result<Vec<u8>, Error>;
    fn next_reg(&mut self) -> Result<Vec<u8>, Error>;
}

impl InstructionEncoder for Assembler {
    fn xyz(x: u8, y: u8, z: u8) -> u8 {
        ((x & 3) << 6) | ((y & 7) << 3) | (z & 7)
    }

    fn xpqz(x: u8, p: u8, q: u8, z: u8) -> u8 {
        ((x & 3) << 6) | ((p & 3) << 4) | ((q & 1) << 3) | (z & 7)
    }

    fn alu(op: AluOp, r: u8) -> u8 {
        (2 << 6) | (((op as u8 & 7) << 3) | r & 7)
    }

    fn rot_encode(op: RotOp, r: u8) -> u8 {
        (op as u8 & 7) << 3 | (r & 7)
    }

    fn alu_imm(op: AluOp) -> u8 {
        (3 << 6) | (((op as u8) << 3) | 6)
    }

    fn alu_op(&mut self, a: AluOp) -> Result<Vec<u8>, Error> {
        match self.next_token()? {
            IndexIndirect(r, n) => return Ok(vec![0xDD | (r as u8 - 4) << 5, Self::alu(a, Reg::_HL_ as u8), n]),
            RegisterIX(r) => return Ok(vec![0xDD, Self::alu(a, r as u8)]),
            RegisterIY(r) => return Ok(vec![0xFD, Self::alu(a, r as u8)]),
            Register(r) => return Ok(vec![Self::alu(a, r as u8)]),
            Number(n) => return Ok(vec![Self::alu_imm(a), n as u8]),
            t @ _ => {
                println!("{:?}", t);
            }
        }
        Err(self.error(ErrorType::InvalidInstruction))
    }

    fn alu_op_r(&mut self, a: AluOp, x: u8, q: u8) -> Result<Vec<u8>, Error> {
        let lhs = self.next_token()?;
        if !self.next_token_is(&Delimiter(Comma)) {
            self.tokens.push(lhs);
            return self.alu_op(a);
        }
        self.expect_token(Delimiter(Comma))?;
        let rhs = self.next_token()?;


        match (&lhs, &rhs, self.z80n_enabled) {
            (RegisterPair(Hl), RegisterPair(reg), _) => match a {
                AluOp::Add => return Ok(vec![Self::xpqz(0, reg.rp1()?, 1, 1)]),
                AluOp::Adc => return Ok(vec![0xED, Self::xpqz(1, reg.rp1()?, 1, 2)]),
                AluOp::Sbc => return Ok(vec![0xED, Self::xpqz(1, reg.rp1()?, 0, 2)]),
                _ => {}
            }

            (RegisterPair(Ix), RegisterPair(rp), _) => return Ok(vec![0xDD, Self::xpqz(x, rp.rp1()?, q, 1)]),
            (RegisterPair(Iy), RegisterPair(rp), _) => return Ok(vec![0xFD, Self::xpqz(x, rp.rp1()?, q, 1)]),
            (Register(Reg::A), _, _) => {
                self.tokens.push(rhs);
                return self.alu_op(a);
            }

            (RegisterPair(_), Register(Reg::A), false) => return Err(self.error(ErrorType::Z80NDisabled)),
            (RegisterPair(_), Number(_), false) => return Err(self.error(ErrorType::Z80NDisabled)),
            (RegisterPair(_), ConstLabel(_), false) => return Err(self.error(ErrorType::Z80NDisabled)),

            (RegisterPair(rp), Register(Reg::A), true) => return Ok(vec![0xED, 0x31 + rp.nrp()?]),
            (RegisterPair(rp), Number(addr), true) => {
                if *addr < 0 || *addr > 65536 {
                    self.warn(ErrorType::AddressTruncated);
                }
                return Ok(vec![0xED, 0x34 + rp.nrp()?, addr.lo(), addr.hi()]);
            }
            (RegisterPair(rp), ConstLabel(l), true) => {
                let addr = self.try_resolve_label(l, 2, false, false);
                return Ok(vec![0xED, 0x34 + rp.nrp()?, addr.lo(), addr.hi()]);
            }
            _ => {}
        }
        return Err(self.error(ErrorType::InvalidInstruction));
    }

    fn bit_res_set(&mut self, x: u8) -> Result<Vec<u8>, Error> {
        let bit = self.expect_number(0..8)?;
        self.expect_token(Delimiter(Comma))?;
        match self.next_token()? {
            RegisterIX(r) => Ok(vec![0xDD, 0xCb, Self::xyz(x, bit as u8, r as u8)]),
            RegisterIY(r) => Ok(vec![0xFD, 0xCb, Self::xyz(x, bit as u8, r as u8)]),
            Register(r) => Ok(vec![0xCb, Self::xyz(x, bit as u8, r as u8)]),
            _ => Err(self.error(ErrorType::InvalidInstruction))
        }
    }

    fn call_jp(&mut self, q: u8, z: u8) -> Result<Vec<u8>, Error> {
        let instr: u8;
        if let Condition(c) = *self.tokens.last().unwrap_or(&Token::None)
        {
            self.tokens.pop();
            self.expect_token(Delimiter(Comma))?;
            instr = Self::xyz(3, c as u8, z - 1);
        } else {
            instr = Self::xpqz(3, 0, q, z);
        }
        if self.tokens.len() > 1 {
            match self.expr.parse(&mut self.tokens, self.current_pc + 1) {
                Ok(Some(addr)) => return Ok(vec![instr, addr.lo(), addr.hi()]),
                Ok(None) => return Ok(vec![instr, 0, 0]),
                Err(e) => return Err(self.error(e))
            }
        }
        let tok = self.next_token()?;
        if let Some(addr) = self.get_address(&tok, 1) {
            return Ok(vec![instr, addr.lo(), addr.hi()]);
        }
        Err(self.error(ErrorType::InvalidInstruction))
    }

    fn jp(&mut self) -> Result<Vec<u8>, Error> {
        if let Some(bytes) = match self.tokens.last() {
            Some(IndexIndirect(i, _)) => {
                let ixy = (*i as u8) - 4 << 5;
                Some(vec![0xDD | ixy, Self::xpqz(3, 2, 1, 1)])
            }
            Some(Register(_HL_)) => {
                Some(vec![Self::xpqz(3, 2, 1, 1)])
            }
            _ => None
        } {
            self.tokens.pop();
            return Ok(bytes);
        }
        self.call_jp(0, 3)
    }

    fn ex(&mut self) -> Result<Vec<u8>, Error> {
        let lhs = &self.next_token()?;
        self.expect_token(Delimiter(Comma))?;
        let rhs = &self.next_token()?;

        match (lhs, rhs) {
            (RegisterPair(Af), RegisterPair(_Af)) => Ok(vec![0x08]),
            (RegisterPair(De), RegisterPair(Hl)) => Ok(vec![0xEB]),
            (RegisterIndirect(RegPairInd::Sp), RegisterPair(Hl)) => Ok(vec![0xE3]),
            (RegisterIndirect(RegPairInd::Sp), RegisterPair(Ix)) => Ok(vec![0xDD, 0xE3]),
            (RegisterIndirect(RegPairInd::Sp), RegisterPair(Iy)) => Ok(vec![0xFD, 0xE3]),
            _ => {
                //self.info(format!("{:?},{:?}", lhs, rhs).as_str());
                Err(self.error(ErrorType::InvalidRegisterPair))
            }
        }
    }

    fn im(&mut self) -> Result<Vec<u8>, Error> {
        if let Number(mut n) = self.next_token()? {
            if (0..3).contains(&n) {
                if n > 0 {
                    n += 1;
                }
                return Ok(vec![0xED, Self::xyz(1, n as u8, 6)]);
            }
            return Err(self.error(ErrorType::IntegerOutOfRange));
        }
        Err(self.error(ErrorType::SyntaxError))
    }

    fn inc_dec(&mut self, q: u8) -> Result<Vec<u8>, Error> {
        match self.next_token()? {
            IndexIndirect(reg, n) => Ok(vec![0xDD | ((reg as u8 - 4) << 5), Self::xyz(0, _HL_ as u8, q + 4), n]),
            RegisterPair(Ix) => Ok(vec![0xDD, Self::xpqz(0, 2, q, 3)]),
            RegisterPair(Iy) => Ok(vec![0xFD, Self::xpqz(0, 2, q, 3)]),
            RegisterPair(r) => Ok(vec![Self::xpqz(0, r.rp1()?, q, 3)]),
            RegisterIX(r) => Ok(vec![0xDD, Self::xyz(0, r as u8, q + 4)]),
            RegisterIY(r) => Ok(vec![0xFD, Self::xyz(0, r as u8, q + 4)]),
            Register(r) => Ok(vec![Self::xyz(0, r as u8, q + 4)]),
            _ => Err(self.error(ErrorType::SyntaxError))
        }
    }

    fn io_op(&mut self, y: u8) -> Result<Vec<u8>, Error> {
        let lhs = &self.next_token()?;

        if !self.next_token_is(&Delimiter(Comma)) && lhs == &RegisterIndirect(RegPairInd::C) {
            if y == 3 {
                return Ok(vec![0xED, 0x70]);
            } else {
                return Err(self.error(ErrorType::SyntaxError));
            }
        }
        self.expect_token(Delimiter(Comma))?;
        let rhs = &self.next_token()?;

        match (&lhs, &rhs, &y) {
            //In
            (Register(Reg::A), AddressIndirect(_), 3) => if let Some(addr) = rhs.number_to_u8() {
                return Ok(vec![Self::xyz(3, y, 3), addr]);
            }
            (Register(r), RegisterIndirect(RegPairInd::C), 3) => {
                let yy = r.clone() as u8;
                return Ok(vec![0xED, Self::xyz(1, yy, 0)]);
            }

            //Out
            (AddressIndirect(_), Register(Reg::A), 2) => if let Some(addr) = lhs.number_to_u8() {
                return Ok(vec![Self::xyz(3, y, 3), addr]);
            }
            (RegisterIndirect(RegPairInd::C), Number(0), 2) => return Ok(vec![0xED, 0x71]),
            (RegisterIndirect(RegPairInd::C), Register(r), 2) => return Ok(vec![0xED, Self::xyz(1, r.clone() as u8, 1)]),
            _ => {}
        }

        Err(self.error(ErrorType::SyntaxError))
    }

    fn jr(&mut self) -> Result<Vec<u8>, Error> {
        let token = self.tokens.last().unwrap_or(&Token::EndOfFile).clone();
        match token {
            Number(_) | ConstLabel(_) => Ok(vec![Self::xyz(0, 3, 0), self.relative()?]),
            Condition(c) => match &c {
                Cnd::Z | Cnd::C | Cnd::Nz | Cnd::NC => {
                    self.next_token()?;
                    self.expect_token(Delimiter(Comma))?;
                    Ok(vec![Self::xyz(0, c.clone() as u8 + 4, 0), self.relative()?])
                }
                _ => Err(self.error(ErrorType::InvalidCondition))
            }
            _ => Err(self.error(ErrorType::SyntaxError))
        }
    }

    fn push_pop(&mut self, z: u8) -> Result<Vec<u8>, Error> {
        match (self.next_token()?, self.z80n_enabled) {
            (RegisterPair(Ix), _) => Ok(vec![0xDD, Self::xpqz(3, 2, 0, z)]),
            (RegisterPair(Iy), _) => Ok(vec![0xFD, Self::xpqz(3, 2, 0, z)]),
            (RegisterPair(r), _) => Ok(vec![Self::xpqz(3, r.rp2()?, 0, z)]),
            (Number(_), false) => Err(self.error(ErrorType::Z80NDisabled)),
            (ConstLabel(_), false) => Err(self.error(ErrorType::Z80NDisabled)),
            (Number(n), true) => {
                if n < 0 || n > 65535 {
                    self.warn(ErrorType::AddressTruncated)
                }
                Ok(vec![0xED, 0x8A, n.hi(), n.lo()])
            }
            (ConstLabel(l), true) => {
                let addr = self.try_resolve_label(&l, 2, false, true);
                Ok(vec![0xED, 0x8A, addr.hi(), addr.lo()])
            }
            _ => Err(self.error(ErrorType::InvalidInstruction))
        }
    }

    fn ret(&mut self) -> Result<Vec<u8>, Error> {
        if self.tokens.len() > 0 {
            let tok = self.next_token()?;
            if let Condition(c) = tok {
                return Ok(vec![Self::xyz(3, c.clone() as u8, 0)]);
            }
            self.tokens.push(tok);
        }
        return Ok(vec![0xC9]);
    }

    fn rot(&mut self, a: RotOp) -> Result<Vec<u8>, Error> {
        match self.next_token()? {
            IndexIndirect(r, n) => return Ok(vec![0xDD | (r as u8 - 4) << 5, Self::rot_encode(a, Reg::_HL_ as u8), n]),
            Register(r) => return Ok(vec![0xCB, Self::rot_encode(a, r as u8)]),
            _ => {}
        }
        Err(self.error(ErrorType::SyntaxError))
    }

    fn rst(&mut self) -> Result<Vec<u8>, Error> {
        if let Number(n) = self.next_token()? {
            if ((n / 8) & 7) * 8 != n {
                return Err(self.error(ErrorType::IntegerOutOfRange));
            }
            return Ok(vec![Self::xyz(3, n as u8 >> 3, 7)]);
        }
        Err(self.error(ErrorType::InvalidInstruction))
    }

    fn load(&mut self) -> Result<Vec<u8>, Error> {
        let lhs = self.next_token()?;
        self.expect_token(Delimiter(Comma))?;
        let rhs = self.next_token()?;

        if lhs.is_special_reg() || rhs.is_special_reg() {
            return self.load_special(&lhs, &rhs);
        }

        if lhs.is_indirect() || rhs.is_indirect() {
            return self.load_indirect(&lhs, &rhs);
        }

        if lhs.is_reg() {
            return self.load_r(&lhs, &rhs);
        }

        if lhs.is_reg_pair() {
            return self.load_rp(&lhs, &rhs);
        }

        Err(self.error(ErrorType::SyntaxError))
    }

    fn load_indirect(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error> {
        let b = match (dst, src) {
            (RegisterPair(Hl), AddressIndirect(a)) => Some(vec![Self::xpqz(0, 2, 1, 2), a.lo(), a.hi()]),
            (RegisterPair(Hl), ConstLabelIndirect(l)) => {
                let a = self.try_resolve_label(l, 1, false, false);
                Some(vec![Self::xpqz(0, 2, 1, 2), a.lo(), a.hi()])
            }
            (RegisterPair(Ix), AddressIndirect(a)) => Some(vec![Self::xpqz(0, 2, 1, 2), a.lo(), a.hi()]),
            (RegisterPair(Ix), ConstLabelIndirect(l)) => {
                let a = self.try_resolve_label(l, 2, false, false);
                Some(vec![Self::xpqz(0, 2, 1, 2), a.lo(), a.hi()])
            }
            (RegisterPair(Iy), AddressIndirect(a)) => Some(vec![Self::xpqz(0, 2, 1, 2), a.lo(), a.hi()]),
            (RegisterPair(Iy), ConstLabelIndirect(l)) => {
                let a = self.try_resolve_label(l, 2, false, false);
                Some(vec![Self::xpqz(0, 2, 1, 2), a.lo(), a.hi()])
            }
            (RegisterPair(r), AddressIndirect(a)) => Some(vec![0xED, Self::xpqz(1, r.rp1().unwrap(), 1, 3), a.lo(), a.hi()]),

            (RegisterIndirect(rp), Register(Reg::A)) => Some(vec![Self::xpqz(0, rp.clone() as u8, 0, 2)]),

            (Register(Reg::A), RegisterIndirect(r)) => Some(vec![Self::xpqz(0, r.clone() as u8, 1, 2)]),
            (Register(Reg::A), AddressIndirect(a)) => Some(vec![Self::xpqz(0, 3, 1, 2), a.lo(), a.hi()]),
            (Register(Reg::A), ConstLabelIndirect(s)) => {
                let a = self.try_resolve_label(s, 1, false, false);
                Some(vec![Self::xpqz(0, 3, 1, 2), a.lo(), a.hi()])
            }
            (AddressIndirect(a), Register(Reg::A)) => Some(vec![Self::xpqz(0, 3, 0, 2), a.lo(), a.hi()]),
            (ConstLabelIndirect(l), Register(Reg::A)) => {
                let a = self.try_resolve_label(l, 1, false, false);
                Some(vec![Self::xpqz(0, 3, 0, 2), a.lo(), a.hi()])
            }
            (AddressIndirect(a), RegisterPair(Hl)) => Some(vec![Self::xpqz(0, 2, 0, 2), a.lo(), a.hi()]),
            (ConstLabelIndirect(l), RegisterPair(Hl)) => {
                let a = self.try_resolve_label(l, 1, false, false);
                Some(vec![Self::xpqz(0, 2, 0, 2), a.lo(), a.hi()])
            }
            (AddressIndirect(a), RegisterPair(Ix)) => Some(vec![Self::xpqz(0, 2, 0, 2), a.lo(), a.hi()]),
            (ConstLabelIndirect(l), RegisterPair(Ix)) => {
                let a = self.try_resolve_label(l, 2, false, false);
                Some(vec![Self::xpqz(0, 2, 0, 2), a.lo(), a.hi()])
            }
            (AddressIndirect(a), RegisterPair(Iy)) => Some(vec![Self::xpqz(0, 2, 0, 2), a.lo(), a.hi()]),
            (ConstLabelIndirect(l), RegisterPair(Iy)) => {
                let a = self.try_resolve_label(l, 2, false, false);
                Some(vec![Self::xpqz(0, 2, 0, 2), a.lo(), a.hi()])
            }
            (AddressIndirect(a), RegisterPair(r)) => Some(vec![0xED, Self::xpqz(1, r.rp1()?, 0, 3), a.lo(), a.hi()]),
            (ConstLabelIndirect(l), RegisterPair(r)) => {
                let a = self.try_resolve_label(l, 2, false, false);
                Some(vec![0xED, Self::xpqz(1, r.rp1()?, 0, 3), a.lo(), a.hi()])
            }

            (Register(r), IndexIndirect(reg, o)) => Some(vec![0xDD | (reg.clone() as u8 - 4) << 5, Self::xyz(1, r.clone() as u8, Reg::_HL_ as u8), o.clone()]),

            (IndexIndirect(rp, i), Number(n)) => Some(vec![0xDD | (rp.clone() as u8 - 4) << 5, 0x36, i.clone(), n.clone() as u8]),
            (IndexIndirect(rp, o), Register(r)) => Some(vec![0xDD | (rp.clone() as u8 - 4) << 5, Self::xyz(1, Reg::_HL_ as u8, r.clone() as u8), o.clone()]),

            _ => {
                println!("load_indirect: {:?},{:?}", dst, src);
                None
            }
        };

        match b {
            None => Err(self.error(ErrorType::SyntaxError)),
            Some(mut b) => {
                if let Some(prefix) = dst.is_index_prefix() {
                    b.insert(0, prefix);
                } else if let Some(prefix) = src.is_index_prefix() {
                    b.insert(0, prefix);
                }
                Ok(b)
            }
        }
        //Err(Error::syntax_error(self.line_number))
    }

    fn load_r(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error> {
        let mut encoded: Vec<u8> = vec![];
        let mut offset = 1;
        // IX or IY?
        if let Some(p) = dst.is_index_prefix().or_else(|| src.is_index_prefix()).or(None) {
            encoded.push(p);
            offset += 1;
        }

        // LD A,(nn)
        match (dst, src) {
            (Register(Reg::A), AddressIndirect(addr)) => {
                encoded.append(&mut vec![Self::xpqz(0, 3, 1, 2), addr.lo(), addr.hi()]);
                return Ok(encoded);
            }
            _ => {}
        }

        // is src a valid 8 bit reg?
        let r = if let Some(n) = dst.reg_value() { n } else {
            return Err(self.error(ErrorType::SyntaxError));
        };

        if let Some(n) = self.decode_number(src, offset)? {
            if n < 0 || n > 255 {
                self.warn(ErrorType::ByteTrunctated);
            }
            encoded.push(Self::xyz(0, r, 6));
            encoded.push(n as u8);
        } else if let Some(rr) = src.reg_value() {
            encoded.push(Self::xyz(1, r, rr));
        } else {
            return Err(self.error(ErrorType::SyntaxError));
        }
        Ok(encoded)
    }

    fn load_rp(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error> {
        //println!("load_rp {:?},{:?}", dst, src);
        let mut encoded: Vec<u8> = vec![];
        let mut instr_size = 1;
        if let Some(p) = dst.is_index_prefix().or_else(|| src.is_index_prefix()).or(None) {
            encoded.push(p);
            instr_size += 1;
        }

        let rp = if let RegisterPair(rp) = dst { rp.rp1()? } else { 0 };

        let addr = match (dst, src) {
            (RegisterPair(_), Number(n)) => *n,
            (RegisterPair(_), ConstLabel(l)) => self.try_resolve_label(l, instr_size, false, false) as isize,
            //(RegisterPair(Hl), LabelIndirect(l)) => self.try_resolve_label(l, instr_size, false, false) as isize,
            //(RegisterPair(Hl), AddressIndirect(a)) => *a as isize,
            _ => return Err(self.error(ErrorType::SyntaxError))
        };

        if src.is_indirect() {
            encoded.append(&mut vec![Self::xpqz(0, 2, 1, 2), addr.lo(), addr.hi()]);
        } else {
            encoded.append(&mut vec![Self::xpqz(0, rp, 0, 1), addr.lo(), addr.hi()]);
        }

        return Ok(encoded);
    }

    fn load_special(&mut self, dst: &Token, src: &Token) -> Result<Vec<u8>, Error> {
        match (dst, src) {
            (RegisterPair(Sp), Number(n)) => if (0..65536).contains(n) {
                let addr = n.clone() as u16;
                return Ok(vec![Self::xpqz(0, 3, 0, 1), addr.lo(), addr.hi()]);
            } else {
                return Err(self.error(ErrorType::IntegerOutOfRange));
            }
            (ConstLabelIndirect(l), RegisterPair(Sp)) => {
                let addr = self.try_resolve_label(l, 2, false, false);
                return Ok(vec![0xED, 0x73, addr.lo(), addr.hi()]);
            }
            (RegisterPair(Sp), ConstLabelIndirect(l)) => {
                let addr = self.try_resolve_label(l, 2, false, false);
                return Ok(vec![0xED, 0x7B, addr.lo(), addr.hi()]);
            }
            (RegisterPair(Sp), RegisterPair(Hl)) => return Ok(vec![Self::xpqz(3, 3, 1, 1)]),
            (RegisterPair(Sp), RegisterPair(Ix)) => return Ok(vec![0xDD, Self::xpqz(3, 3, 1, 1)]),
            (RegisterPair(Sp), RegisterPair(Iy)) => return Ok(vec![0xFD, Self::xpqz(3, 3, 1, 1)]),
            (AddressIndirect(a), RegisterPair(Sp)) => return Ok(vec![0xED, 0x73, a.lo(), a.hi()]),
            (RegisterPair(Sp), AddressIndirect(a)) => return Ok(vec![0xED, 0x7B, a.lo(), a.hi()]),
            _ => {
                //self.info(format!("{:?},{:?}", dst, src).as_ref());
            }
        }

        let y = match (dst, src) {
            (RegisterIR(Ir::I), Register(Reg::A)) => Some(0),
            (RegisterIR(Ir::R), Register(Reg::A)) => Some(1),
            (Register(Reg::A), RegisterIR(Ir::I)) => Some(2),
            (Register(Reg::A), RegisterIR(Ir::R)) => Some(3),
            _ => None
        };

        if y.is_none() {
            return Err(self.error(ErrorType::InvalidInstruction));
        }

        Ok(vec![0xED, Self::xyz(1, y.unwrap(), 7)])
    }

    fn mul(&mut self) -> Result<Vec<u8>, Error> {
        if !self.z80n_enabled {
            return Err(self.error(ErrorType::Z80NDisabled));
        }
        self.expect_token(Register(Reg::D))?;
        self.expect_token(Delimiter(Comma))?;
        self.expect_token(Register(Reg::E))?;
        Ok(vec![0xED, 0x30])
    }

    fn next_reg(&mut self) -> Result<Vec<u8>, Error> {
        if !self.z80n_enabled {
            return Err(self.error(ErrorType::Z80NDisabled));
        }
        let reg = self.get_byte()?;
        self.expect_token(Delimiter(Comma))?;
        match self.next_token()? {
            Register(Reg::A) => Ok(vec![0xED, 0x92, reg]),
            Number(n) => {
                if n > 256 || n < 0 {
                    self.warn(ByteTrunctated);
                }
                Ok(vec![0xED, 0x91, reg, n as u8])
            }
            _ => Err(self.error(InvalidInstruction))
        }
    }
}