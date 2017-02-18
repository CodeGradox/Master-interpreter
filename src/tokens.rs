use std::fmt;

use tokens::Token::*;
use real::Real;

/// Represents a valid token returned by `Lexer::get_token`
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Types
    Int(i32),
    Real(Real),
    Str(String),
    Nil,

    // Identifier
    Identity(String),

    // Assignment
    Assignment,
    PlusAssignment,
    MinusAssignment,
    MulAssignment,
    DivAssignment,

    // Arithmetic
    Plus,
    Minus,
    Mul,
    Div,

    // Comparison
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,

    // Logic
    And,
    Or,
    Not,

    // Brackets
    LeftParam,
    RightParam,
    LeftCurlyParam,
    RightCurlyParam,
    LeftSquareParam,
    RightSquareParam,

    // Keywords
    At, // @
    By,
    Function,
    True,
    False,
    If,
    Else,
    While,
    For,
    Break,
    Return,
    QuestionMark,

    // Misc
    Semicolon,
    Comma,
    Dot,

    // Ranges
    ExclusiveRange,
    InclusiveRange,

    EndOfFile,
}

impl Token {
    /// Is this a keyword token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_keyword(&self) -> bool {
        match *self {
              At
            | Function
            | True
            | False
            | If
            | Else
            | While
            | For
            | Break
            | Return
            | QuestionMark => true,
            _ => false,
        }
    }

    /// Is this an assignment token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_assignment(&self) -> bool {
        match *self {
              Assignment
            | PlusAssignment
            | MinusAssignment
            | MulAssignment
            | DivAssignment => true,
            _ => false,
        }
    }

    /// Is this an arithmetic  token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_arithmetic(&self) -> bool {
        match *self {
              Plus
            | Minus
            | Mul
            | Div => true,
            _ => false,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:?}", self)
        match *self {
            Int(i) => write!(f, "Int: {}", i),
            Real(r) => write!(f, "Real: {}", r),
            Str(ref s) => write!(f, "Str: \"{}\"", s),
            Identity(ref i) => write!(f, "Identity: \"{}\"", i),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Performs a check on the input str `id` to see
/// whenever it is a keyword token or a name token
/// and then returns the coresponding `Token`.
pub fn lookup_identity(id: String) -> Token {
    match id.as_str() {
        "fn" => Function,
        "true" => True,
        "false" => False,
        "if" => If,
        "else" => Else,
        "while" => While,
        "for" => For,
        "break" => Break,
        "return" => Return,
        "nil" => Nil,
        _ => Identity(id),
    }
}
