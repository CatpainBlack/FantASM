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
    Align,
    Macro,
    End,
    StringZero,
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

    // cspect
    Break,
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptionType {
    Verbose,
    CSpect,
    Z80n,
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
    IndexIndirect(RegPair, u8),
    Condition(Cnd),
    StringLiteral(String),
    Opt(OptionType),
    Boolean(bool),
    IndirectExpression(Vec<Token>),
    MacroParam(String),
}