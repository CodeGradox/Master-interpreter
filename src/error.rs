use std::fmt;
use std::num;
use std::error::Error as StdError;

use error::Error::*;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
/// The error types of the interpreter.
pub enum Error {
    InfiniteString,
    StringEOL,
    LargeInt,
    BadRealLiteral,
    Illegal(char),
    UnknownEscape(char),
    ParseIntError(num::ParseIntError),

    TempParseErr,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Illegal(c) => write!(f, "{} {}", self.description(), c),
            UnknownEscape(c) => {
                write!(f,
                       "{} {}",
                       self.description(),
                       c.escape_default().collect::<String>())
            }
            ParseIntError(ref e) => fmt::Display::fmt(e, f),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            InfiniteString => "infinite string literal",
            StringEOL => "found newline in string literal",
            LargeInt => "int literal too big",
            BadRealLiteral => "could not parse real literal",
            Illegal(_) => "found illegal character",
            UnknownEscape(_) => "found unknow escape code",
            ParseIntError(ref e) => e.description(),
            TempParseErr => "parser error",
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        ParseIntError(err)
    }
}
