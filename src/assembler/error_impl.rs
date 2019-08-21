extern crate colour;

use std::error::Error as StdErr;
use std::fmt;
use std::fmt::Formatter;

use crate::assembler::{Error, ErrorLevel};

pub enum ErrorType {
    //InternalError,
    InvalidLabel,
    LabelNotFound,
    FileNotFound,
    SyntaxError,
    BadConstant,
    InvalidRegisterPair,
    InvalidInstruction,
    IntegerOutOfRange,
    IntegerExpected,
    AddressTruncated,
    ByteTrunctated,
    LabelOrConstantExists,
    UnexpectedEndOfLine,
    InvalidCondition,
    BadOperator,
    DivideByZero,
    Z80NDisabled,
    UnexpectedClose,
    UnclosedParentheses,
    CSpectDisabled,
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            ErrorType::InvalidLabel => String::from("Invalid character in label"),
            ErrorType::SyntaxError => String::from("Syntax error"),
            ErrorType::BadConstant => String::from("Bad constant definition"),
            ErrorType::InvalidRegisterPair => String::from("Invalid register pair"),
            ErrorType::InvalidInstruction => String::from("Invalid instruction"),
            ErrorType::IntegerOutOfRange => String::from("Integer out of range"),
            ErrorType::IntegerExpected => String::from("Integer expected"),
            ErrorType::FileNotFound => String::from("File not found"),
            ErrorType::LabelNotFound => String::from("Undefined label or constant"),
            ErrorType::AddressTruncated => String::from("Address is out of range, the value has been truncated"),
            ErrorType::LabelOrConstantExists => String::from("Attempt to redefine label or constant"),
            ErrorType::UnexpectedEndOfLine => String::from("Unexpected end of line"),
            ErrorType::InvalidCondition => String::from("Invalid condition"),
            ErrorType::BadOperator => String::from("Bad operator in expression"),
            ErrorType::DivideByZero => String::from("Expression resulted in a divide by zero"),
            ErrorType::Z80NDisabled => String::from("Z80n extended instructions are not enabled"),
            ErrorType::ByteTrunctated => String::from("Integer has been truncated to 8 bits"),
            ErrorType::UnexpectedClose => String::from("Unexpected closing parentheses"),
            ErrorType::UnclosedParentheses => String::from("Unclosed parentheses"),
            ErrorType::CSpectDisabled => String::from("CSpect pseudo ops are not enabled"),
        }
    }
}

impl Error {
    pub fn fatal(message: &str, line_no: isize) -> Error {
        Error {
            line_no,
            message: message.to_string(),
            level: ErrorLevel::Fatal,
            file_name: "replace me".to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: line {} - {}", self.level, self.line_no, self.message)
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
            line_no: 0,
            message: s,
            level: ErrorLevel::Fatal,
            file_name: "fantasm".to_string(),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error {
            line_no: 0,
            message: e.description().to_string(),
            level: ErrorLevel::Fatal,
            file_name: "fantasm".to_string(),
        }
    }
}