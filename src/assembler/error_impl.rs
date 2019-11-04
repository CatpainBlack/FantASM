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
    //UnhandledDirective,
    DanglingEnd,
    EndIfWithoutIf,
    ElseWithoutIf,
    //ExpectedParenthesis,

    BadMacroName,
    CommaExpected,
    BadMacroParam,
    NestedMacro,
    MacroParamCount,
    MacroLabel,
    MacroExists,

    CodeSize,
    UnknownSizeOf,

    NonAscii,
    //NotImplemented,

    RegisterExpected,

    EnumBadName,
    EnumBadEnd,
    EnumMemberName,
    EnumStepValue,

}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            ErrorType::BadExpression => String::from("Invalid number or expression"),
            ErrorType::InvalidInstruction => String::from("Invalid instruction"),
            ErrorType::BitTruncated => String::from("Bit number is out of range will and will be truncated"),
            ErrorType::RegisterExpected => String::from("Invalid 8-bit register"),
            ErrorType::InvalidRegisterPair => String::from("Invalid register pair"),
            ErrorType::PCOverflow => String::from("Address overflow, PC > 65535"),
            ErrorType::InvalidLabel => String::from("Invalid character in label"),
            ErrorType::BadConstant => String::from("Bad constant definition"),
            ErrorType::IntegerOutOfRange => String::from("Integer out of range"),
            ErrorType::FileNotFound => String::from("File not found"),
            ErrorType::LabelNotFound => String::from("Undefined label or constant"),
            ErrorType::AddressTruncated => String::from("Address is out of range, the value has been truncated"),
            ErrorType::LabelOrConstantExists => String::from("Attempt to redefine label or constant"),
            ErrorType::InvalidCondition => String::from("Invalid condition"),
            ErrorType::Z80NDisabled => String::from("Z80n extended instructions are not enabled"),
            ErrorType::ByteTruncated => String::from("Integer has been truncated to 8 bits"),

            ErrorType::SyntaxError => String::from("Syntax error"),
            ErrorType::UnexpectedEndOfLine => String::from("Unexpected end of line"),
            ErrorType::UnexpectedClose => String::from("Unexpected closing parentheses"),
            ErrorType::UnclosedParentheses => String::from("Unclosed parentheses"),
            ErrorType::CSpectDisabled => String::from("CSpect pseudo ops are not enabled"),
            ErrorType::InvalidOption => String::from("Invalid assembler option"),
            ErrorType::WordTruncated => String::from("Integer has been truncated to 16 bits"),
            ErrorType::HexStringExpected => String::from("Invalid Hexadecimal string"),
            ErrorType::MultipleIncludes => String::from("Source file previously included"),
            ErrorType::ExtraCharacters => String::from("Discarded extra characters at and of line"),
            ErrorType::DanglingEnd => String::from("Encountered END without MACRO directive "),
            ErrorType::BadMacroName => String::from("Invalid or missing macro name"),
            ErrorType::CommaExpected => String::from("Comma expected"),
            ErrorType::BadMacroParam => String::from("Invalid or missing macro parameter name"),
            ErrorType::NestedMacro => String::from("Macros may not be nested"),
            ErrorType::MacroParamCount => String::from("Incorrect number of macro parameters"),
            ErrorType::MacroLabel => String::from("Only local labels are permitted inside macros"),
            ErrorType::NonAscii => String::from("String contains non-ascii characters"),
            ErrorType::CodeSize => String::from("Maximum code size exceeded"),
            ErrorType::EndIfWithoutIf => String::from("ENDIF without IF"),
            ErrorType::ElseWithoutIf => String::from("ELSE without IF"),
            ErrorType::UnknownSizeOf => String::from("SizeOf cannot be determined"),
            ErrorType::MacroExists => String::from("Macro already defined"),

            ErrorType::EnumBadName => String::from("ENUM name expected"),
            ErrorType::EnumBadEnd => String::from("ENDE without ENUM"),
            ErrorType::EnumMemberName => String::from("Enum member name is invalid"),
            ErrorType::EnumStepValue => String::from("Enum step value cannot be zero"),
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