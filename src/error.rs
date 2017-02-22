use std::fmt;
use std::num;
use std::error::Error as StdError;

use error::Error::*;
use error::ParseErrorKind::*;
use source_pos::SourcePos;

pub type Result<T> = ::std::result::Result<T, Error>;

/// The error types of the interpreter.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ParseError(ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            ParseError(ref e) => e.description(),
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        ParseError(err)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    // Lexer error
    InfiniteString,
    StringEOL,
    LargeInt,
    BadRealLiteral,
    Illegal(char),
    UnknownEscape(char),
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),

    // Parser error
    TempParseErr,
    ExpectedPrimitive,
    ExpectedAtom(String),
    EndOfFile(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub pos: SourcePos,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, pos: SourcePos) -> Self {
        ParseError {
            kind: kind,
            pos: pos,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.pos)?;
        match self.kind {
            Illegal(c) => write!(f, "{} {}", self.description(), c),
            UnknownEscape(c) => {
                write!(f,
                       "{} {}",
                       self.description(),
                       c.escape_default().collect::<String>())
            }
            ParseIntError(ref e) => fmt::Display::fmt(e, f),
            ParseFloatError(ref e) => fmt::Display::fmt(e, f),
            EndOfFile(ref s) =>
                write!(f, "{} found {}", s, self.description()),
            ExpectedAtom(ref s) =>
                write!(f, "{} found {}", self.description(), s),
            _ => f.write_str(self.description()),
        }
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        match self.kind {
            InfiniteString => "infinite string literal",
            StringEOL => "newline in string literal",
            LargeInt => "int literal too big",
            BadRealLiteral => "could not parse real literal",
            Illegal(_) => "illegal character",
            UnknownEscape(_) => "unknow escape code",
            ParseIntError(ref e) => e.description(),
            ParseFloatError(ref e) => e.description(),
            TempParseErr => "parser error",
            ExpectedPrimitive => "expected primitive",
            EndOfFile(_) => "end of file",
            ExpectedAtom(_) => "expected atom",
        }
    }
}

impl From<num::ParseIntError> for ParseErrorKind {
    fn from(err: num::ParseIntError) -> Self {
        ParseIntError(err)
    }
}

impl From<num::ParseFloatError> for ParseErrorKind {
    fn from(err: num::ParseFloatError) -> Self {
        ParseFloatError(err)
    }
}
