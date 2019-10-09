use crate::assembler::tokens::{Cnd, Del, Directive, Ir, IxU, IyU, Op, OpCode, OptionType, Reg, RegPair, RegPairInd, Token};

impl ToString for OpCode {
    fn to_string(&self) -> String {
        match self {
            OpCode::Nop => "nop",
            OpCode::Adc => "adc",
            OpCode::Add => "add",
            OpCode::And => "add",
            OpCode::Bit => "bit",
            OpCode::Call => "call",
            OpCode::Ccf => "ccf",
            OpCode::Cp => "cp",
            OpCode::Cpd => "cpd",
            OpCode::Cpdr => "cpdr",
            OpCode::Cpi => "cpi",
            OpCode::Cpir => "cpir",
            OpCode::Cpl => "cpl",
            OpCode::Daa => "daa",
            OpCode::Dec => "dec",
            OpCode::Di => "di",
            OpCode::Djnz => "djnz",
            OpCode::Ei => "ei",
            OpCode::Ex => "ex",
            OpCode::Exx => "exx",
            OpCode::Halt => "halt",
            OpCode::Im => "im",
            OpCode::In => "in",
            OpCode::Inc => "inc",
            OpCode::Ind => "ind",
            OpCode::Indr => "indr",
            OpCode::Ini => "ini",
            OpCode::Inir => "inir",
            OpCode::Jr => "jr",
            OpCode::Jp => "jp",
            OpCode::Ld => "ld",
            OpCode::Ldd => "ldd",
            OpCode::Lddr => "lddr",
            OpCode::Ldi => "ldi",
            OpCode::Ldir => "ldir",
            OpCode::Neg => "neg",
            OpCode::Or => "or",
            OpCode::Otdr => "otdr",
            OpCode::Otir => "otir",
            OpCode::Out => "out",
            OpCode::Outd => "outd",
            OpCode::Outi => "outi",
            OpCode::Pop => "pop",
            OpCode::Push => "push",
            OpCode::Res => "res",
            OpCode::Ret => "ret",
            OpCode::Reti => "reti",
            OpCode::Retn => "retn",
            OpCode::Rl => "rl",
            OpCode::Rla => "rla",
            OpCode::Rlc => "rlc",
            OpCode::Rlca => "rlca",
            OpCode::Rld => "rld",
            OpCode::Rr => "rr",
            OpCode::Rra => "rrca",
            OpCode::Rrc => "rrc",
            OpCode::Rrca => "rrca",
            OpCode::Rrd => "rrd",
            OpCode::Rst => "rst",
            OpCode::Sbc => "sbc",
            OpCode::Scf => "scf",
            OpCode::Set => "set",
            OpCode::Sla => "sla",
            OpCode::Sll => "sll",
            OpCode::Sra => "sra",
            OpCode::Srl => "srl",
            OpCode::Sub => "sub",
            OpCode::Xor => "xor",
            OpCode::Ldix => "ldix",
            OpCode::Ldws => "ldws",
            OpCode::Ldirx => "ldirx",
            OpCode::Lddx => "lddx",
            OpCode::Lddrx => "lddrx",
            OpCode::Ldpirx => "ldpirx",
            OpCode::Outinb => "outinb",
            OpCode::Mul => "mul",
            OpCode::Swapnib => "swapnib",
            OpCode::Mirror => "mirror",
            OpCode::Nextreg => "nextreg",
            OpCode::Pixeldn => "pixeldn",
            OpCode::Pixelad => "pixelad",
            OpCode::Setae => "setae",
            OpCode::Test => "test",
            OpCode::Break => "break",
            OpCode::Exit => "exit"
        }.to_string()
    }
}

impl ToString for Directive {
    fn to_string(&self) -> String {
        match self {
            Directive::Org => "ORG",
            Directive::Include => "INCLUDE",
            Directive::Binary => "INCBIN",
            Directive::Message => "!message",
            Directive::Opt => "!opt",
            Directive::Byte => "DB",
            Directive::Word => "DW",
            Directive::Block => "DS",
            Directive::Hex => "DH",
            Directive::Align => "ALIGN",
            Directive::Macro => "MACRO",
            Directive::End => "END",
            Directive::StringZero => "DZ",
        }.to_string()
    }
}

impl ToString for RegPair {
    fn to_string(&self) -> String {
        match self {
            RegPair::Bc => "bc",
            RegPair::De => "de",
            RegPair::Hl => "hl",
            RegPair::Sp => "sp",
            RegPair::Ix => "ix",
            RegPair::Iy => "iy",
            RegPair::Af => "af",
            RegPair::_Af => "af'",
        }.to_string()
    }
}

impl ToString for Ir {
    fn to_string(&self) -> String {
        match self {
            Ir::I => "I",
            Ir::R => "R",
        }.to_string()
    }
}

impl ToString for Reg {
    fn to_string(&self) -> String {
        match self {
            Reg::B => "b",
            Reg::C => "c",
            Reg::D => "d",
            Reg::E => "e",
            Reg::H => "h",
            Reg::L => "l",
            Reg::_HL_ => "(hl)",
            Reg::A => "a"
        }.to_string()
    }
}

impl ToString for IxU {
    fn to_string(&self) -> String {
        match self {
            IxU::Ixh => "ixh",
            IxU::Ixl => "ixl",
        }.to_string()
    }
}

impl ToString for IyU {
    fn to_string(&self) -> String {
        match self {
            IyU::Iyh => "iyh",
            IyU::Iyl => "iyl",
        }.to_string()
    }
}

impl ToString for Del {
    fn to_string(&self) -> String {
        match self {
            Del::Comma => ",",
        }.to_string()
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Shl => "<<",
            Op::Shr => ">>",
            Op::Lt => "<",
            Op::Gt => ">",
            Op::LParens => "(",
            Op::RParens => ")",
            Op::Equals => "=",
            Op::Ampersand => "&",
            Op::Pipe => "|",
            Op::AsmPc => "$"
        }.to_string()
    }
}

impl ToString for RegPairInd {
    fn to_string(&self) -> String {
        match self {
            RegPairInd::Bc => "(bc)",
            RegPairInd::De => "(de)",
            RegPairInd::Sp => "(sp)",
            RegPairInd::C => "(c)",
        }.to_string()
    }
}

impl ToString for Cnd {
    fn to_string(&self) -> String {
        match self {
            Cnd::Nz => "nz",
            Cnd::Z => "z",
            Cnd::NC => "nc",
            Cnd::C => "c",
            Cnd::PO => "po",
            Cnd::PE => "pe",
            Cnd::P => "p",
            Cnd::M => "m",
        }.to_string()
    }
}

impl ToString for OptionType {
    fn to_string(&self) -> String {
        match self {
            OptionType::Verbose => "verbose",
            OptionType::CSpect => "cspect",
            OptionType::Z80n => "z80n",
        }.to_string()
    }
}


impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::None => "None".to_string(),
            Token::Invalid => "Invalid".to_string(),
            Token::EndOfFile => "EOF".to_string(),
            Token::ConstLabel(l) => l.to_string(),
            Token::Directive(d) => d.to_string(),
            Token::OpCode(o) => o.to_string(),
            Token::Number(n) => format!("{}", n).to_string(),
            Token::RegisterPair(rp) => rp.to_string(),
            Token::RegisterIR(ir) => ir.to_string(),
            Token::Register(r) => r.to_string(),
            Token::RegisterIX(ixu) => ixu.to_string(),
            Token::RegisterIY(iyu) => iyu.to_string(),
            Token::Delimiter(del) => del.to_string(),
            Token::Operator(op) => op.to_string(),
            Token::RegisterIndirect(ri) => ri.to_string(),
            Token::IndexIndirect(r, i) => format!("({}+{})", r.to_string(), i),
            Token::Condition(c) => c.to_string(),
            Token::StringLiteral(l) => l.to_string(),
            Token::Opt(o) => o.to_string(),
            Token::Boolean(b) => format!("{}", b),
            Token::IndirectExpression(i) => {
                let t: Vec<String> = i.into_iter().map(|e| e.to_string()).collect();
                t.join(" ")
            }
            Token::MacroParam(mp) => mp.to_string(),
            Token::IndexIndirectExpression(_, _) => unimplemented!()
        }
    }
}