use std::error::Error as E;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    OsError,
    UnknownError,
    ExecError,
    UnicodeDecodeError,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    code: Option<i32>,
    details: String,
}

impl Error {
    pub fn new(kind: ErrorKind, code: Option<i32>, msg: &str) -> Error {
        Error {
            kind: kind,
            code: code,
            details: msg.to_string(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        return self.kind;
    }

    pub fn code(&self) -> Option<i32> {
        return self.code;
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl E for Error {
    fn description(&self) -> &str {
        &self.details
    }
}
