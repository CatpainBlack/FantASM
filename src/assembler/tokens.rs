#[repr(usize)]
#[derive(Debug, Clone, PartialEq)]
pub enum RegPairInd {
    Bc = 0,
    De = 1,
    Sp = 2,
    C = 3,
}

#[repr(usize)]
#[derive(Debug, Clone, PartialEq)]
pub enum Ir {
    I = 8,
    R = 9,
}


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Cnd {
    Nz = 0,
    Z = 1,
    NC = 2,
    C = 3,
    PO = 4,
    PE = 5,
    P = 6,
    M = 7,
}

#[repr(usize)]
#[derive(Debug, Clone, PartialEq)]
pub enum IxU {
    Ixh = 4,
    Ixl = 5,
}

#[repr(usize)]
#[derive(Debug, Clone, PartialEq)]
pub enum IyU {
    Iyh = 4,
    Iyl = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Reg {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    _HL_ = 6,
    A = 7,
}

#[repr(usize)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RegPair {
    Bc = 0,
    De = 1,
    Hl = 2,
    Sp = 3,
    Ix = 4,
    Iy = 5,
    Af = 6,
    _Af = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AluOp {
    Add = 0,
    Adc = 1,
    Sub = 2,
    Sbc = 3,
    And = 4,
    Xor = 5,
    Or = 6,
    Cp = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RotOp {
    Rlc = 0,
    Rrc = 1,
    Rl = 2,
    Rr = 3,
    Sla = 4,
    Sra = 5,
    Sll = 6,
    Srl = 7,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Directive {
    Org,
    Include,
    Binary,
    Message,
    Opt,
    Byte,
    Word,
    Block,
    Hex,
    Define,
    Macro,
    End,
    StringZero,
    If,
    IfDef,
    IfNotDef,
    Else,
    EndIf,
    Global,
    Enum,
    EndEnum,
    Struct,
    EndStruct,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Del {
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Shl,
    Shr,
    Lt,
    Gt,
    LParens,
    RParens,
    Equals,
    Ampersand,
    Pipe,
    AsmPc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Functions {
    SizeOf(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Nop,
    Adc,
    Add,
    And,
    Bit,
    Call,
    Ccf,
    Cp,
    Cpd,
    Cpdr,
    Cpi,
    Cpir,
    Cpl,
    Daa,
    Dec,
    Di,
    Djnz,
    Ei,
    Ex,
    Exx,
    Halt,
    Im,
    In,
    Inc,
    Ind,
    Indr,
    Ini,
    Inir,
    Jr,
    Jp,
    Ld,
    Ldd,
    Lddr,
    Ldi,
    Ldir,
    Neg,
    Or,
    Otdr,
    Otir,
    Out,
    Outd,
    Outi,
    Pop,
    Push,
    Res,
    Ret,
    Reti,
    Retn,
    Rl,
    Rla,
    Rlc,
    Rlca,
    Rld,
    Rr,
    Rra,
    Rrc,
    Rrca,
    Rrd,
    Rst,
    Sbc,
    Scf,
    Set,
    Sla,
    Sll,
    Sra,
    Srl,
    Sub,
    Xor,

    // Z80n
    Ldix,
    Ldws,
    Ldirx,
    Lddx,
    Lddrx,
    Ldpirx,
    Outinb,
    Mul,
    Swapnib,
    Mirror,
    Nextreg,
    Pixeldn,
    Pixelad,
    Setae,
    Test,
    Bsla,
    Bsra,
    Bsrl,
    Bsrf,
    Brlc,

    // cspect
    Break,
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
    Verbose,
    CSpect,
    Z80n,
    MaxCodeSize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Bool {
    True,
    False,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    None,
    Invalid,
    EndOfFile,
    ConstLabel(String),
    Directive(Directive),
    OpCode(OpCode),
    Number(isize),
    RegisterPair(RegPair),
    RegisterIR(Ir),
    Register(Reg),
    RegisterIX(IxU),
    RegisterIY(IyU),
    Delimiter(Del),
    Operator(Op),
    RegisterIndirect(RegPairInd),
    IndexIndirect(RegPair, Vec<Token>),
    Condition(Cnd),
    StringLiteral(String),
    Opt(OptionType),
    Boolean(bool),
    IndirectExpression(Vec<Token>),
    MacroParam(String),
    Function(Functions),
}