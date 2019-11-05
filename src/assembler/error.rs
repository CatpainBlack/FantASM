use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ErrorLevel {
    Fatal,
}

pub struct Error {
    pub line_no: isize,
    pub message: String,
    pub level: ErrorLevel,
    pub file_name: String,
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
        use std::error::Error;
        self::Error {
            line_no: -1,
            message: e.description().to_string(),
            level: ErrorLevel::Fatal,
            file_name: "FantASM".to_string(),
        }
    }
}