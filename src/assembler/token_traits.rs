use std::str::FromStr;

use ascii::AsAsciiStr;
use regex::Regex;

use crate::assembler::number_parser::NumberParser;
use crate::assembler::tokens::{AluOp, Bool, OptionType, RegPair, RotOp, Token};
use crate::assembler::tokens::{Cnd, Del, Directive, Ir, IxU, IyU, Op, OpCode, Reg};
use crate::assembler::tokens::Token::{ConstLabel, Function, IndexIndirect, IndirectExpression, Number, Operator, Register, RegisterIndirect, RegisterIR, RegisterIX, RegisterIY, RegisterPair};
use crate::assembler::zx_ascii::ZXAscii;

lazy_static! {
    static ref LABEL: Regex = Regex::new(r"^\.?\w*:?$").unwrap();
}

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

impl FromStr for Bool {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "on" | "true" | "yes" => Ok(Bool::True),
            "off" | "false" | "no" => Ok(Bool::False),
            _ => Err(())
        }
    }
}

impl FromStr for OptionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "verbose" => Ok(OptionType::Verbose),
            "cspect" => Ok(OptionType::CSpect),
            "z80n" => Ok(OptionType::Z80n),
            "maxcodesize" => Ok(OptionType::MaxCodeSize),
            _ => Err(())
        }
    }
}

impl FromStr for Ir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "i" => Ok(Ir::I),
            "r" => Ok(Ir::R),
            _ => Err(())
        }
    }
}

impl FromStr for Cnd {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nz" => Ok(Cnd::Nz),
            "z" => Ok(Cnd::Z),
            "nc" => Ok(Cnd::NC),
            "c" => Ok(Cnd::C),
            "po" => Ok(Cnd::PO),
            "pe" => Ok(Cnd::PE),
            "p" => Ok(Cnd::P),
            "m" => Ok(Cnd::M),
            _ => Err(())
        }
    }
}

impl FromStr for IxU {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ixh" => Ok(IxU::Ixh),
            "ixl" => Ok(IxU::Ixl),
            _ => Err(())
        }
    }
}

impl FromStr for IyU {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "iyh" => Ok(IyU::Iyh),
            "iyl" => Ok(IyU::Iyl),
            _ => Err(())
        }
    }
}

impl FromStr for Reg {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "b" => Ok(Reg::B),
            "c" => Ok(Reg::C),
            "d" => Ok(Reg::D),
            "e" => Ok(Reg::E),
            "h" => Ok(Reg::H),
            "l" => Ok(Reg::L),
            "_hl_" => Ok(Reg::_HL_),
            "a" => Ok(Reg::A),
            _ => Err(())
        }
    }
}

impl FromStr for RegPair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bc" => Ok(RegPair::Bc),
            "de" => Ok(RegPair::De),
            "hl" => Ok(RegPair::Hl),
            "sp" => Ok(RegPair::Sp),
            "ix" => Ok(RegPair::Ix),
            "iy" => Ok(RegPair::Iy),
            "af" => Ok(RegPair::Af),
            "af'" => Ok(RegPair::_Af),
            _ => Err(())
        }
    }
}

impl FromStr for AluOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "add" => Ok(AluOp::Add),
            "adc" => Ok(AluOp::Adc),
            "sub" => Ok(AluOp::Sub),
            "sbc" => Ok(AluOp::Sbc),
            "and" => Ok(AluOp::And),
            "xor" => Ok(AluOp::Xor),
            "or" => Ok(AluOp::Or),
            "cp" => Ok(AluOp::Cp),
            _ => Err(())
        }
    }
}

impl FromStr for RotOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rlc" => Ok(RotOp::Rlc),
            "rrc" => Ok(RotOp::Rrc),
            "rl" => Ok(RotOp::Rl),
            "rr" => Ok(RotOp::Rr),
            "sla" => Ok(RotOp::Sla),
            "sra" => Ok(RotOp::Sra),
            "sll" => Ok(RotOp::Sll),
            "srl" => Ok(RotOp::Srl),
            _ => Err(())
        }
    }
}

impl FromStr for Directive {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "org" => Ok(Directive::Org),
            "include" => Ok(Directive::Include),
            "binary" | "incbin" => Ok(Directive::Binary),
            "!message" => Ok(Directive::Message),
            "db" | "defb" | "byte" => Ok(Directive::Byte),
            "dw" | "defw" | "word" => Ok(Directive::Word),
            "ds" | "block" => Ok(Directive::Block),
            "dh" | "hex" => Ok(Directive::Hex),
            "!opt" | "#pragma" => Ok(Directive::Opt),
            //"align" => Ok(Directive::Align),
            "macro" => Ok(Directive::Macro),
            "end" | "endm" => Ok(Directive::End),
            "dz" => Ok(Directive::StringZero),
            "#if" | "if" => Ok(Directive::If),
            "#ifdef" | "ifdef" => Ok(Directive::IfDef),
            "#ifndef" | "ifndef" => Ok(Directive::IfNotDef),
            "#else" | "else" => Ok(Directive::Else),
            "#endif" | "endif" => Ok(Directive::EndIf),
            "global" => Ok(Directive::Global),
            "#define" => Ok(Directive::Define),
            "enum" => Ok(Directive::Enum),
            "ende" => Ok(Directive::EndEnum),
            "struct" => Ok(Directive::Struct),
            "ends" => Ok(Directive::EndStruct),
            _ => Err(())
        }
    }
}

impl FromStr for Del {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "," => Ok(Del::Comma),
            _ => Err(())
        }
    }
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "<<" => Ok(Op::Shl),
            ">>" => Ok(Op::Shr),
            "<" => Ok(Op::Lt),
            ">" => Ok(Op::Gt),
            "(" => Ok(Op::LParens),
            ")" => Ok(Op::RParens),
            "equ" | "=" => Ok(Op::Equals),
            "&" => Ok(Op::Ampersand),
            "|" => Ok(Op::Pipe),
            "$" | "asmpc" => Ok(Op::AsmPc),
            _ => Err(())
        }
    }
}

impl Tokens for Token {
    fn from_string(word: String) -> Token {
        let w = word.to_lowercase();

        // string literals
        let is_double_quoted = word.starts_with("\"") & &word.ends_with("\"");
        let is_single_quoted = word.starts_with("\'") & &word.ends_with("\'");

        if is_single_quoted && word.chars().count() == 3 {
            let zx_string = ZXAscii::zx_safe(&word);
            let zx_ascii = match zx_string.as_ascii_str() {
                Ok(s) => s,
                Err(_e) => "   ".as_ascii_str().unwrap(),
            };
            return Token::Number(zx_ascii.as_bytes()[1] as isize);
        }

        if is_single_quoted || is_double_quoted {
            if let Some(s) = word.get(1..word.len() - 1) {
                return Token::StringLiteral(s.to_string());
            }
        }

        // Directives
        if let Ok(d) = Directive::from_str(&w) {
            return Token::Directive(d);
        }
        //Opcodes
        if let Ok(o) = OpCode::from_str(&w) {
            return Token::OpCode(o);
        }
        // Numbers
        if let Some(n) = word.to_number() {
            return Token::Number(n as isize);
        }
        //Register pairs
        if let Ok(rp) = RegPair::from_str(&w) {
            return Token::RegisterPair(rp);
        }
        // Registers
        if let Ok(r) = Reg::from_str(&w) {
            return Token::Register(r);
        }

        // I/R Register
        if let Ok(r) = Ir::from_str(&w) {
            return Token::RegisterIR(r);
        }

        // Delimiters
        if let Ok(d) = Del::from_str(&w) {
            return Token::Delimiter(d);
        }
        // Operators
        if let Ok(op) = Op::from_str(&w) {
            return Token::Operator(op);
        }
        // IHx/IXh
        if let Ok(ixu) = IxU::from_str(&w) {
            return Token::RegisterIX(ixu);
        }
        // IYx/IYh
        if let Ok(iyu) = IyU::from_str(&w) {
            return Token::RegisterIY(iyu);
        }

        // Conditions
        if let Ok(cnd) = Cnd::from_str(&w) {
            return Token::Condition(cnd);
        }

        // Options
        if let Ok(opt) = OptionType::from_str(&w) {
            return Token::Opt(opt);
        }

        // Boolean/Truth
        if let Ok(t) = Bool::from_str(&w) {
            return Token::Boolean(t == Bool::True);
        }

        // Label
        if LABEL.is_match_at(&word, 0) {
            return Token::ConstLabel(word);
        }

        Token::Invalid
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
            //AddressIndirect(_) => true,
            IndexIndirect(_, _) => true,
            IndirectExpression(_) => true,
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
        match self {
            RegisterPair(_) => true,
            _ => false
        }
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
            ConstLabel(_) => true,
            Function(_) => true,
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
//            AddressIndirect(n) => if (0..256).contains(n) {
//                Some(n.clone() as u8)
//            } else { None }
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

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nop" => Ok(OpCode::Nop),
            "adc" => Ok(OpCode::Adc),
            "add" => Ok(OpCode::Add),
            "and" => Ok(OpCode::And),
            "bit" => Ok(OpCode::Bit),
            "call" => Ok(OpCode::Call),
            "ccf" => Ok(OpCode::Ccf),
            "cp" => Ok(OpCode::Cp),
            "cpd" => Ok(OpCode::Cpd),
            "cpdr" => Ok(OpCode::Cpdr),
            "cpi" => Ok(OpCode::Cpi),
            "cpir" => Ok(OpCode::Cpir),
            "cpl" => Ok(OpCode::Cpl),
            "daa" => Ok(OpCode::Daa),
            "dec" => Ok(OpCode::Dec),
            "di" => Ok(OpCode::Di),
            "djnz" => Ok(OpCode::Djnz),
            "ei" => Ok(OpCode::Ei),
            "ex" => Ok(OpCode::Ex),
            "exx" => Ok(OpCode::Exx),
            "halt" => Ok(OpCode::Halt),
            "im" => Ok(OpCode::Im),
            "in" => Ok(OpCode::In),
            "inc" => Ok(OpCode::Inc),
            "ind" => Ok(OpCode::Ind),
            "indr" => Ok(OpCode::Indr),
            "ini" => Ok(OpCode::Ini),
            "inir" => Ok(OpCode::Inir),
            "jr" => Ok(OpCode::Jr),
            "jp" => Ok(OpCode::Jp),
            "ld" => Ok(OpCode::Ld),
            "ldd" => Ok(OpCode::Ldd),
            "lddr" => Ok(OpCode::Lddr),
            "ldi" => Ok(OpCode::Ldi),
            "ldir" => Ok(OpCode::Ldir),
            "neg" => Ok(OpCode::Neg),
            "or" => Ok(OpCode::Or),
            "otdr" => Ok(OpCode::Otdr),
            "otir" => Ok(OpCode::Otir),
            "out" => Ok(OpCode::Out),
            "outd" => Ok(OpCode::Outd),
            "outi" => Ok(OpCode::Outi),
            "pop" => Ok(OpCode::Pop),
            "push" => Ok(OpCode::Push),
            "res" => Ok(OpCode::Res),
            "ret" => Ok(OpCode::Ret),
            "reti" => Ok(OpCode::Reti),
            "retn" => Ok(OpCode::Retn),
            "rl" => Ok(OpCode::Rl),
            "rla" => Ok(OpCode::Rla),
            "rlc" => Ok(OpCode::Rlc),
            "rlca" => Ok(OpCode::Rlca),
            "rld" => Ok(OpCode::Rld),
            "rr" => Ok(OpCode::Rr),
            "rra" => Ok(OpCode::Rra),
            "rrc" => Ok(OpCode::Rrc),
            "rrca" => Ok(OpCode::Rrca),
            "rrd" => Ok(OpCode::Rrd),
            "rst" => Ok(OpCode::Rst),
            "sbc" => Ok(OpCode::Sbc),
            "scf" => Ok(OpCode::Scf),
            "set" => Ok(OpCode::Set),
            "sla" => Ok(OpCode::Sla),
            "sll" => Ok(OpCode::Sll),
            "sra" => Ok(OpCode::Sra),
            "srl" => Ok(OpCode::Srl),
            "sub" => Ok(OpCode::Sub),
            "xor" => Ok(OpCode::Xor),

            // z80n
            "ldix" => Ok(OpCode::Ldix),
            "ldws" => Ok(OpCode::Ldws),
            "ldirx" => Ok(OpCode::Ldirx),
            "lddx" => Ok(OpCode::Lddx),
            "lddrx" => Ok(OpCode::Lddrx),
            "ldpirx" => Ok(OpCode::Ldpirx),
            "outinb" => Ok(OpCode::Outinb),
            "mul" => Ok(OpCode::Mul),
            "swapnib" => Ok(OpCode::Swapnib),
            "mirror" => Ok(OpCode::Mirror),
            "nextreg" => Ok(OpCode::Nextreg),
            "pixeldn" => Ok(OpCode::Pixeldn),
            "pixelad" => Ok(OpCode::Pixelad),
            "setae" => Ok(OpCode::Setae),
            "test" => Ok(OpCode::Test),

            // cspect
            "break" => Ok(OpCode::Break),
            "exit" => Ok(OpCode::Exit),
            _ => Err(())
        }
    }
}

