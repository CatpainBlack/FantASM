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

extern crate colour;

use std::error::Error as StdErr;
use std::fmt;
use std::fmt::Formatter;

use crate::assembler::{Error, ErrorLevel};

pub enum ErrorType {
    PCOverflow,
    InvalidLabel,
    LabelNotFound,
    FileNotFound,
    SyntaxError,
    BadConstant,
    InvalidRegisterPair,
    InvalidInstruction,
    IntegerOutOfRange,
    AddressTruncated,
    ByteTruncated,
    WordTruncated,
    LabelOrConstantExists,
    UnexpectedEndOfLine,
    InvalidCondition,
    Z80NDisabled,
    UnexpectedClose,
    UnclosedParentheses,
    CSpectDisabled,
    InvalidOption,
    BadExpression,
    HexStringExpected,
    BitTruncated,
    MultipleIncludes,
    ExtraCharacters,
    UnhandledDirective,
    DanglingEnd,

    BadMacroName,
    CommaExpected,
    BadMacroParam,
    NestedMacro,
    MacroParamCount,
    MacroLabel,

    CodeSize,

    NonAscii,
    //NotImplemented,
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            //ErrorType::NotImplemented => String::from("Not implemented"),
            ErrorType::PCOverflow => String::from("Address overflow, PC > 65535"),
            ErrorType::InvalidLabel => String::from("Invalid character in label"),
            ErrorType::SyntaxError => String::from("Syntax error"),
            ErrorType::BadConstant => String::from("Bad constant definition"),
            ErrorType::InvalidRegisterPair => String::from("Invalid register pair"),
            ErrorType::InvalidInstruction => String::from("Invalid instruction"),
            ErrorType::IntegerOutOfRange => String::from("Integer out of range"),
            ErrorType::FileNotFound => String::from("File not found"),
            ErrorType::LabelNotFound => String::from("Undefined label or constant"),
            ErrorType::AddressTruncated => String::from("Address is out of range, the value has been truncated"),
            ErrorType::LabelOrConstantExists => String::from("Attempt to redefine label or constant"),
            ErrorType::UnexpectedEndOfLine => String::from("Unexpected end of line"),
            ErrorType::InvalidCondition => String::from("Invalid condition"),
            ErrorType::Z80NDisabled => String::from("Z80n extended instructions are not enabled"),
            ErrorType::ByteTruncated => String::from("Integer has been truncated to 8 bits"),
            ErrorType::UnexpectedClose => String::from("Unexpected closing parentheses"),
            ErrorType::UnclosedParentheses => String::from("Unclosed parentheses"),
            ErrorType::CSpectDisabled => String::from("CSpect pseudo ops are not enabled"),
            ErrorType::InvalidOption => String::from("Invalid assembler option"),
            ErrorType::BadExpression => String::from("Unable to parse expression"),
            ErrorType::WordTruncated => String::from("Integer has been truncated to 16 bits"),
            ErrorType::HexStringExpected => String::from("Invalid Hexadecimal string"),
            ErrorType::BitTruncated => String::from("Bit number is out of range will and will be truncated"),
            ErrorType::MultipleIncludes => String::from("Source file previously included"),
            ErrorType::ExtraCharacters => String::from("Discarded extra characters at and of line"),
            ErrorType::UnhandledDirective => String::from("Unhandled directive"),
            ErrorType::DanglingEnd => String::from("Encountered END without MACRO directive "),
            ErrorType::BadMacroName => String::from("Invalid or missing macro name"),
            ErrorType::CommaExpected => String::from("Comma expected"),
            ErrorType::BadMacroParam => String::from("Invalid or missing macro parameter name"),
            ErrorType::NestedMacro => String::from("Macros may not be nested"),
            ErrorType::MacroParamCount => String::from("Incorrect number of macro parameters"),
            ErrorType::MacroLabel => String::from("Only local labels are permitted inside macros"),
            ErrorType::NonAscii => String::from("Bad string"),
            ErrorType::CodeSize => String::from("Maximum code size exceeded")
        }
    }
}

impl Error {
    pub fn fatal(message: &str, line_no: isize, file_name: &str) -> Error {
        Error {
            line_no,
            message: message.to_string(),
            level: ErrorLevel::Fatal,
            file_name: file_name.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.line_no > -1 {
            write!(f, "{:?}: line {} - {}", self.level, self.line_no, self.message)
        } else {
            write!(f, "{:?} - {}", self.level, self.message)
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: line {} - {}", self.level, self.line_no, self.message)
    }
}

impl std::convert::From<std::string::String> for Error {
    fn from(s: String) -> Self {
        Error {
            line_no: -1,
            message: s,
            level: ErrorLevel::Fatal,
            file_name: "FantASM".to_string(),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            line_no: -1,
            message: e.description().to_string(),
            level: ErrorLevel::Fatal,
            file_name: "FantASM".to_string(),
        }
    }
}