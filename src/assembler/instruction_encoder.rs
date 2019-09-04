use crate::assembler::{Assembler, Error};
use crate::assembler::error_impl::ErrorType;
use crate::assembler::reg_pair::HighLow;
use crate::assembler::reg_pair::RegPairValue;
use crate::assembler::token_traits::Tokens;
use crate::assembler::tokens::{AluOp, Cnd, Ir, Reg, RegPairInd, RotOp, Token};
use crate::assembler::tokens::Del::Comma;
use crate::assembler::tokens::Reg::_HL_;
use crate::assembler::tokens::RegPair::{_Af, Af, De, Hl, Ix, Iy, Sp};
use crate::assembler::tokens::Token::{AddressIndirect, Condition, Delimiter, IndexIndirect, ConstLabel, Number, Register, RegisterIndirect, RegisterIR, RegisterIX, RegisterIY, RegisterPair, Operator, IndirectExpression};
use crate::assembler::tokens::Op::{LParens, RParens};

macro_rules! xyz {
    ($x: expr, $y: expr, $z: expr) => {
        (($x & 3) << 6) | (($y & 7) << 3) | ($z & 7)
    }
}

macro_rules! xpqz {
    ($x: expr, $p: expr, $q: expr, $z: expr) => {
        (($x & 3) << 6) | (($p & 3) << 4) | (($q & 1) << 3) | ($z & 7)
    }
}

macro_rules! alu {
    ($op: expr, $r: expr) => {
        (2 << 6) | ((($op as u8 & 7) << 3) | $r & 7)
    }
}
macro_rules! rot_encode {
    ($op: expr, $r: expr) => {
        ($op as u8 & 7) << 3 | ($r & 7)
    }
}

macro_rules! alu_imm {
    ($op: expr) => {
        (3 << 6) | ((($op as u8) << 3) | 6)
    }
}

pub(crate) trait InstructionEncoder {
    fn alu_op(&mut self, a: AluOp) -> Result<(), Error>;
    fn alu_op_r(&mut self, a: AluOp, x: u8, q: u8) -> Result<(), Error>;
    fn bit_res_set(&mut self, x: u8) -> Result<(), Error>;
    fn call_jp(&mut self, q: u8, z: u8) -> Result<(), Error>;
    fn jp(&mut self) -> Result<(), Error>;
    fn ex(&mut self) -> Result<(), Error>;
    fn im(&mut self) -> Result<(), Error>;
    fn inc_dec(&mut self, q: u8) -> Result<(), Error>;
    fn io_op(&mut self, y: u8) -> Result<(), Error>;
    fn jr(&mut self, djnz: bool) -> Result<(), Error>;
    fn push_pop(&mut self, z: u8) -> Result<(), Error>;
    fn ret(&mut self) -> Result<(), Error>;
    fn rot(&mut self, a: RotOp) -> Result<(), Error>;
    fn rst(&mut self) -> Result<(), Error>;
    fn load(&mut self) -> Result<(), Error>;
    fn load_indirect(&mut self, dst: &Token, src: &Token) -> Result<(), Error>;
    fn load_r(&mut self, dst: &Token, src: &Token) -> Result<(), Error>;
    fn load_rp(&mut self, dst: &Token, src: &Token) -> Result<(), Error>;
    fn load_special(&mut self, dst: &Token, src: &Token) -> Result<(), Error>;
    fn mul(&mut self) -> Result<(), Error>;
    fn next_reg(&mut self) -> Result<(), Error>;
    fn indirect_expression(&mut self) -> Result<Token, Error>;
}

impl InstructionEncoder for Assembler {
    fn alu_op(&mut self, a: AluOp) -> Result<(), Error> {
        let tok = self.take_token()?;

        self.context.pc_add(self.bank.emit_prefix(&tok));

        match tok {
            IndexIndirect(_, n) => self.emit(&[alu!(a, Reg::_HL_ as u8), n]),
            RegisterIX(r) => self.emit_byte(alu!(a, r as u8)),
            RegisterIY(r) => self.emit_byte(alu!(a, r as u8)),
            Register(r) => self.emit_byte(alu!(a, r as u8)),
            _ => {
                self.tokens.push(tok);
                let b = self.expect_byte(1)? as u8;
                self.emit(&[alu_imm!(a), b])
            }
        }
    }

    fn alu_op_r(&mut self, a: AluOp, x: u8, q: u8) -> Result<(), Error> {
        let lhs = self.take_token()?;

        if self.expect_token(Delimiter(Comma)).is_err() {
            self.tokens.push(lhs);
            return self.alu_op(a);
        }

        let rhs = self.take_token()?;

        self.context.pc_add(self.bank.emit_prefix(&lhs));

        match (&lhs, &rhs, self.z80n_enabled) {
            (RegisterPair(Hl), RegisterPair(reg), _) => match a {
                AluOp::Add => return self.emit_byte(xpqz!(0, reg.rp1()?, 1, 1)),
                AluOp::Adc => return self.emit(&[0xED, xpqz!(1, reg.rp1()?, 1, 2)]),
                AluOp::Sbc => return self.emit(&[0xED, xpqz!(1, reg.rp1()?, 0, 2)]),
                _ => {}
            }

            (RegisterPair(Ix), RegisterPair(rp), _) | (RegisterPair(Iy), RegisterPair(rp), _) => return self.emit_byte(xpqz!(x, rp.rp1()?, q, 1)),
            (Register(Reg::A), _, _) => {
                self.tokens.push(rhs);
                return self.alu_op(a);
            }

            (RegisterPair(_), Register(Reg::A), false) => return Err(self.context.error(ErrorType::Z80NDisabled)),
            (RegisterPair(_), Number(_), false) => return Err(self.context.error(ErrorType::Z80NDisabled)),
            (RegisterPair(_), ConstLabel(_), false) => return Err(self.context.error(ErrorType::Z80NDisabled)),

            (RegisterPair(rp), Register(Reg::A), true) => return self.emit(&[0xED, 0x31 + rp.nrp()?]),
            (RegisterPair(rp), Number(addr), true) => {
                self.emit(&[0xED, 0x34 + rp.nrp()?])?;
                return self.emit_word(*addr);
            }
            (RegisterPair(rp), ConstLabel(l), true) => {
                self.emit(&[0xED, 0x34 + rp.nrp()?])?;
                let addr = self.context.try_resolve_label(l, 0, false);
                return self.emit_word(addr as isize);
            }
            _ => {}
        }
        return Err(self.context.error(ErrorType::InvalidInstruction));
    }

    fn bit_res_set(&mut self, x: u8) -> Result<(), Error> {
        // todo, fix if iX or Iy
        let bit = self.expect_byte(1)?;
        self.expect_token(Delimiter(Comma))?;
        match self.take_token()? {
            //RegisterIX(r) => self.emit(&[0xDD, 0xCb, xyz!(x, bit as u8, r as u8)])?,
            //RegisterIY(r) => self.emit(&[0xFD, 0xCb, xyz!(x, bit as u8, r as u8)])?,
            Register(r) => self.emit(&[0xCb, xyz!(x, bit as u8, r as u8)])?,
            _ => return Err(self.context.error(ErrorType::InvalidInstruction))
        };
        Ok(())
    }

    fn call_jp(&mut self, q: u8, z: u8) -> Result<(), Error> {
        let instr: u8;
        if let Condition(c) = *self.tokens.last().unwrap_or(&Token::None)
        {
            self.tokens.pop();
            self.expect_token(Delimiter(Comma))?;
            instr = xyz!(3, c as u8, z - 1);
        } else {
            instr = xpqz!(3, 0, q, z);
        }

        let addr = self.expect_word(1)?;
        self.emit(&[instr, addr.lo(), addr.hi()])
    }

    fn jp(&mut self) -> Result<(), Error> {
        if let Some(bytes) = match self.tokens.last() {
            Some(IndexIndirect(i, _)) => {
                let ixy = (*i as u8) - 4 << 5;
                Some(vec![0xDD | ixy, xpqz!(3, 2, 1, 1)])
            }
            Some(Register(_HL_)) => {
                Some(vec![xpqz!(3, 2, 1, 1)])
            }
            _ => None
        } {
            self.tokens.pop();
            return self.emit(&bytes);
        }
        self.call_jp(0, 3)
    }

    fn ex(&mut self) -> Result<(), Error> {
        let lhs = &self.take_token()?;
        self.expect_token(Delimiter(Comma))?;
        let rhs = &self.take_token()?;

        self.context.pc_add(self.bank.emit_prefix(&rhs));

        match (lhs, rhs) {
            (RegisterPair(Af), RegisterPair(_Af)) => self.emit_byte(0x08),
            (RegisterPair(De), RegisterPair(Hl)) => self.emit_byte(0xEB),
            (RegisterIndirect(RegPairInd::Sp), RegisterPair(Hl)) |
            (RegisterIndirect(RegPairInd::Sp), RegisterPair(Ix)) |
            (RegisterIndirect(RegPairInd::Sp), RegisterPair(Iy)) => self.emit_byte(0xE3),
            _ => {
                Err(self.context.error(ErrorType::InvalidRegisterPair))
            }
        }
    }

    fn im(&mut self) -> Result<(), Error> {
        // ToDo: Allow constants?
        if let Number(mut n) = self.take_token()? {
            if (0..3).contains(&n) {
                if n > 0 {
                    n += 1;
                }
                return self.emit(&[0xED, xyz!(1, n as u8, 6)]);
            }
            return Err(self.context.error(ErrorType::IntegerOutOfRange));
        }
        Err(self.context.error(ErrorType::SyntaxError))
    }

    fn inc_dec(&mut self, q: u8) -> Result<(), Error> {
        let tok = self.take_token()?;
        self.context.pc_add(self.bank.emit_prefix(&tok));

        match tok {
            IndexIndirect(_reg, n) => self.emit(&[xyz!(0, _HL_ as u8, q + 4), n]),
            RegisterPair(Ix) => self.emit_byte(xpqz!(0, 2, q, 3)),
            RegisterPair(Iy) => self.emit_byte(xpqz!(0, 2, q, 3)),
            RegisterPair(r) => self.emit_byte(xpqz!(0, r.rp1()?, q, 3)),
            RegisterIX(r) => self.emit_byte(xyz!(0, r as u8, q + 4)),
            RegisterIY(r) => self.emit_byte(xyz!(0, r as u8, q + 4)),
            Register(r) => self.emit_byte(xyz!(0, r as u8, q + 4)),
            _ => Err(self.context.error(ErrorType::SyntaxError))
        }
    }

    fn io_op(&mut self, y: u8) -> Result<(), Error> {
        let lhs = &self.take_token()?;

        if !self.next_token_is(&Delimiter(Comma)) && lhs == &RegisterIndirect(RegPairInd::C) {
            if y == 3 {
                return self.emit(&[0xED, 0x70]);
            } else {
                return Err(self.context.error(ErrorType::SyntaxError));
            }
        }
        self.expect_token(Delimiter(Comma))?;
        let rhs = &self.take_token()?;

        match (&lhs, &rhs, &y) {
            //In
            (Register(Reg::A), AddressIndirect(_), 3) => if let Some(addr) = rhs.number_to_u8() {
                return self.emit(&[xyz!(3, y, 3), addr]);
            }
            (Register(r), RegisterIndirect(RegPairInd::C), 3) => {
                let yy = r.clone() as u8;
                return self.emit(&[0xED, xyz!(1, yy, 0)]);
            }

            //Out
            (AddressIndirect(_), Register(Reg::A), 2) => if let Some(addr) = lhs.number_to_u8() {
                return self.emit(&[xyz!(3, y, 3), addr]);
            }
            (RegisterIndirect(RegPairInd::C), Number(0), 2) => return self.emit(&[0xED, 0x71]),
            (RegisterIndirect(RegPairInd::C), Register(r), 2) => return self.emit(&[0xED, xyz!(1, r.clone() as u8, 1)]),
            _ => {}
        }

        Err(self.context.error(ErrorType::SyntaxError))
    }

    fn jr(&mut self, djnz: bool) -> Result<(), Error> {
        let token = self.tokens.last().unwrap_or(&Token::EndOfFile).clone();
        match token {
            Number(_) | ConstLabel(_) => {
                let offset = self.relative()?;
                if djnz {
                    return self.emit(&[0x10, offset]);
                }
                return self.emit(&[xyz!(0, 3, 0), offset]);
            }
            Condition(c) => match &c {
                Cnd::Z | Cnd::C | Cnd::Nz | Cnd::NC => {
                    self.take_token()?;
                    self.expect_token(Delimiter(Comma))?;
                    let offset = self.relative()?;
                    return self.emit(&[xyz!(0, c.clone() as u8 + 4, 0), offset]);
                }
                _ => Err(self.context.error(ErrorType::InvalidCondition))
            }
            _ => Err(self.context.error(ErrorType::SyntaxError))
        }
    }

    fn push_pop(&mut self, z: u8) -> Result<(), Error> {
        let tok = self.take_token()?;
        self.context.pc_add(self.bank.emit_prefix(&tok));
        match tok {
            RegisterPair(r) => self.emit_byte(xpqz!(3, r.rp2()?, 0, z)),
            _ => if self.z80n_enabled {
                self.tokens.push(tok);
                let n = self.expect_word(2)?;
                self.emit(&[0xED, 0x8A, n.hi(), n.lo()])
            } else {
                Err(self.context.error(ErrorType::InvalidInstruction))
            }
        }
    }

    fn ret(&mut self) -> Result<(), Error> {
        if self.tokens.len() > 0 {
            let tok = self.take_token()?;
            if let Condition(c) = tok {
                return self.emit_byte(xyz!(3, c.clone() as u8, 0));
            }
            self.tokens.push(tok);
        }
        self.emit_byte(0xC9)
    }

    fn rot(&mut self, a: RotOp) -> Result<(), Error> {
        let tok = self.take_token()?;
        self.context.pc_add(self.bank.emit_prefix(&tok));
        match &tok {
            IndexIndirect(_r, n) => self.emit(&[rot_encode!(a, Reg::_HL_ as u8), *n]),
            Register(r) => self.emit(&[0xCB, rot_encode!(a, r.clone() as u8)]),
            _ => Err(self.context.error(ErrorType::SyntaxError))
        }
    }

    fn rst(&mut self) -> Result<(), Error> {
        // ToDo: Allow constants?
        if let Number(n) = self.take_token()? {
            if ((n / 8) & 7) * 8 != n {
                return Err(self.context.error(ErrorType::IntegerOutOfRange));
            }
            self.emit_byte(xyz!(3, n as u8 >> 3, 7))
        } else {
            Err(self.context.error(ErrorType::InvalidInstruction))
        }
    }

    fn load(&mut self) -> Result<(), Error> {
        // ToDo: Handle expressions
        let lhs = self.indirect_expression()?;
        self.expect_token(Delimiter(Comma))?;
        let rhs = self.indirect_expression()?;

        //println!("{:04?}: load {:?},{:?}", self.context.current_line_number(), lhs, rhs);

        if self.bank.emit_prefix(&lhs) == 1 {
            self.context.pc_add(1);
        } else if self.bank.emit_prefix(&rhs) == 1 {
            self.context.pc_add(1);
        }

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

        Err(self.context.error(ErrorType::SyntaxError))
    }

    fn load_indirect(&mut self, dst: &Token, src: &Token) -> Result<(), Error> {
        // ToDo: Handle expressions

        let dst = &match dst {
            RegisterPair(Ix) => RegisterPair(Hl),
            RegisterPair(Iy) => RegisterPair(Hl),
            _ => dst.clone()
        };

        let src = &match src {
            RegisterPair(Ix) => RegisterPair(Hl),
            RegisterPair(Iy) => RegisterPair(Hl),
            _ => src.clone()
        };

        match (dst, src) {
            (RegisterPair(Hl), AddressIndirect(a)) => self.emit(&[xpqz!(0, 2, 1, 2), a.lo(), a.hi()]),
//            (RegisterPair(Hl), ConstLabelIndirect(l)) => {
//                self.emit_byte(xpqz!(0, 2, 1, 2))?;
//                let addr = self.context.try_resolve_label(l, 0, false) as isize;
//                self.emit_word(addr)
//            }
            (RegisterPair(Hl), IndirectExpression(tokens)) => self.emit_instr(None, xpqz!(0, 2, 1, 2), tokens.as_slice()),

            (RegisterPair(r), AddressIndirect(a)) => self.emit(&[0xED, xpqz!(1, r.rp1().unwrap(), 1, 3), a.lo(), a.hi()]),
//            (RegisterPair(r), ConstLabelIndirect(l)) => {
//                self.emit(&[0xED, xpqz!(1, r.rp1().unwrap(), 1, 3)])?;
//                let addr = self.context.try_resolve_label(l, 0, false) as isize;
//                self.emit_word(addr)
//            }
            (RegisterPair(r), IndirectExpression(tokens)) => self.emit_instr(Some(0xED), xpqz!(1, r.rp1().unwrap(), 1, 3), tokens.as_slice()),

            (RegisterIndirect(rp), Register(Reg::A)) => self.emit(&[xpqz!(0, rp.clone() as u8, 0, 2)]),
            (AddressIndirect(a), Register(Reg::A)) => self.emit(&[xpqz!(0, 3, 0, 2), a.lo(), a.hi()]),
//            (ConstLabelIndirect(l), Register(Reg::A)) => {
//                self.emit_byte(xpqz!(0, 3, 0, 2))?;
//                let a = self.context.try_resolve_label(l, 0, false) as isize;
//                self.emit_word(a)
//            }
            (IndirectExpression(tokens), Register(Reg::A)) => self.emit_instr(None, xpqz!(0, 3, 0, 2), tokens.as_slice()),

            (Register(Reg::A), RegisterIndirect(r)) => self.emit(&[xpqz!(0, r.clone() as u8, 1, 2)]),
            (Register(Reg::A), AddressIndirect(a)) => self.emit(&[xpqz!(0, 3, 1, 2), a.lo(), a.hi()]),
//            (Register(Reg::A), ConstLabelIndirect(s)) => {
//                self.emit_byte(xpqz!(0, 3, 1, 2))?;
//                let a = self.context.try_resolve_label(s, 0, false) as isize;
//                self.emit_word(a)
//            }
            (Register(Reg::A), IndirectExpression(tokens)) => self.emit_instr(None, xpqz!(0, 3, 1, 2), tokens.as_slice()),

            (AddressIndirect(a), RegisterPair(Hl)) => self.emit(&[xpqz!(0, 2, 0, 2), a.lo(), a.hi()]),

            (IndirectExpression(tokens), RegisterPair(Hl)) => self.emit_instr(None, xpqz!(0, 2, 0, 2), tokens.as_slice()),

//            (ConstLabelIndirect(l), RegisterPair(Hl)) => {
//                self.emit_byte(xpqz!(0, 2, 0, 2))?;
//                let a = self.context.try_resolve_label(l, 0, false) as isize;
//                self.emit_word(a)
//            }

            (AddressIndirect(a), RegisterPair(r)) => self.emit(&[0xED, xpqz!(1, r.rp1()?, 0, 3), a.lo(), a.hi()]),
//            (ConstLabelIndirect(l), RegisterPair(r)) => {
//                self.emit(&[0xED, xpqz!(1, r.rp1()?, 0, 3)])?;
//                let a = self.context.try_resolve_label(l, 0, false) as isize;
//                self.emit_word(a)
//            }
            (IndirectExpression(tokens), RegisterPair(r)) => self.emit_instr(Some(0xED), xpqz!(1, r.rp1()?, 0, 3), tokens.as_slice()),

            (Register(r), IndexIndirect(_reg, o)) => self.emit(&[xyz!(1, r.clone() as u8, Reg::_HL_ as u8), o.clone()]),

            (IndexIndirect(_rp, i), Number(n)) => self.emit(&[0x36, i.clone(), n.clone() as u8]),
            (IndexIndirect(_rp, o), Register(r)) => self.emit(&[xyz!(1, Reg::_HL_ as u8, r.clone() as u8), o.clone()]),

            _ => {
                println!("load_indirect: {:?},{:?}", dst, src);
                Err(self.context.error(ErrorType::InvalidInstruction))
            }
        }
    }

    fn load_r(&mut self, dst: &Token, src: &Token) -> Result<(), Error> {
        // ToDo: Handle expressions
        let r = match (dst, src) {
            (Register(Reg::A), AddressIndirect(addr)) => {
                self.emit_byte(xpqz!(0, 3, 1, 2))?;
                return self.emit_word(*addr as isize);
            }
            _ => if let Some(n) = dst.reg_value() { n } else {
                return Err(self.context.error(ErrorType::SyntaxError));
            }
        };

        if let Some(n) = self.decode_number(src)? {
            if n < 0 || n > 255 {
                self.warn(ErrorType::ByteTrunctated);
            }
            return self.emit(&[xyz!(0, r, 6), n as u8]);
        } else if let Some(rr) = src.reg_value() {
            return self.emit_byte(xyz!(1, r, rr));
        } else {
            return Err(self.context.error(ErrorType::SyntaxError));
        }
    }

    fn load_rp(&mut self, dst: &Token, src: &Token) -> Result<(), Error> {
        let rp = if let RegisterPair(rp) = dst { rp.rp1()? } else { 0 };
        self.tokens.push(src.clone());
        if src.is_indirect() {
            self.emit_byte(xpqz!(0, 2, 1, 2))?;
        } else {
            self.emit_byte(xpqz!(0, rp, 0, 1))?;
        }
        let addr = match self.expr.parse(&mut self.context, &mut self.tokens, 0, 2) {
            Ok(Some(a)) => a,
            Ok(None) => 0,
            Err(e) => return Err(self.context.error(e))
        };
        self.emit_word(addr)
    }

    fn load_special(&mut self, dst: &Token, src: &Token) -> Result<(), Error> {
        //println!("load_special {:?},{:?}", dst, src);
        match (dst, src) {
            (RegisterPair(Sp), Number(n)) => if (0..65536).contains(n) {
                let addr = n.clone() as u16;
                return self.emit(&[xpqz!(0, 3, 0, 1), addr.lo(), addr.hi()]);
            } else {
                return Err(self.context.error(ErrorType::IntegerOutOfRange));
            }
            (IndirectExpression(l), RegisterPair(Sp)) => return self.emit_instr(Some(0xED), 0x73, l.as_slice()),
            (RegisterPair(Sp), IndirectExpression(l)) => return self.emit_instr(Some(0xED), 0x7B, l.as_slice()),
            (RegisterPair(Sp), RegisterPair(Hl)) => return self.emit(&[xpqz!(3, 3, 1, 1)]),
            (RegisterPair(Sp), RegisterPair(Ix)) => return self.emit(&[xpqz!(3, 3, 1, 1)]),
            (RegisterPair(Sp), RegisterPair(Iy)) => return self.emit(&[xpqz!(3, 3, 1, 1)]),
            (AddressIndirect(a), RegisterPair(Sp)) => return self.emit(&[0xED, 0x73, a.lo(), a.hi()]),
            (RegisterPair(Sp), AddressIndirect(a)) => return self.emit(&[0xED, 0x7B, a.lo(), a.hi()]),
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
            return Err(self.context.error(ErrorType::InvalidInstruction));
        }

        self.emit(&[0xED, xyz!(1, y.unwrap(), 7)])
    }

    fn mul(&mut self) -> Result<(), Error> {
        if !self.z80n_enabled {
            return Err(self.context.error(ErrorType::Z80NDisabled));
        }
        self.expect_token(Register(Reg::D))?;
        self.expect_token(Delimiter(Comma))?;
        self.expect_token(Register(Reg::E))?;
        self.emit(&[0xED, 0x30])
    }

    fn next_reg(&mut self) -> Result<(), Error> {
        if !self.z80n_enabled {
            return Err(self.context.error(ErrorType::Z80NDisabled));
        }
        let reg = self.expect_byte(2)? as u8;
        self.expect_token(Delimiter(Comma))?;
        if let Some(Register(Reg::A)) = self.tokens.last() {
            self.tokens.pop();
            return self.emit(&[0xED, 0x92, reg]);
        }
        let n = self.expect_byte(2)? as u8;
        self.emit(&[0xED, 0x91, reg, n as u8])
    }

    fn indirect_expression(&mut self) -> Result<Token, Error> {
        let lhs = self.take_token()?;
        if lhs == Operator(LParens) {
            let (_, mut tokens) = self.expr.get_expression(&mut self.context, &mut self.tokens);
            match tokens.last() {
                Some(Operator(RParens)) => tokens.pop(),
                _ => return Err(self.context.error(ErrorType::UnclosedParentheses))
            };
            tokens.reverse();
            return Ok(IndirectExpression(tokens));
        }
        Ok(lhs)
    }
}