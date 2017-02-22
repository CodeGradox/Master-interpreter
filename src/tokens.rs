use std::fmt;

use tokens::TokenKind::*;
use real::Real;
use source_pos::SourcePos;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: SourcePos,
}

/// Represents a valid token returned by `Lexer::get_token`
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
    pub fn new(kind: TokenKind, pos: SourcePos) -> Self {
        Token {
            kind: kind,
            pos: pos,
        }
    }

    pub fn line(&self) -> u32 {
        self.pos.line()
    }

    pub fn col(&self) -> u32 {
        self.pos.col()
    }

    /// Is this a keyword token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_keyword(&self) -> bool {
        match self.kind {
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
        match self.kind {
              Assignment
            | PlusAssignment
            | MinusAssignment
            | MulAssignment
            | DivAssignment => true,
            _ => false,
        }
    }

    /// Is this an arithmetic token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_arithmetic(&self) -> bool {
        match self.kind {
              Plus
            | Minus
            | Mul
            | Div => true,
            _ => false,
        }
    }

    /// Is this a literal token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_literal(&self) -> bool {
        match self.kind {
              Int(_)
            | Real(_)
            | Str(_)
            | Nil
            | True
            | False => true,
            _ => false,
        }
    }

    /// Is this a name token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_identity(&self) -> bool {
        match self.kind {
            Identity(_) => true,
            _ => false,
        }
    }

    /// Is this an end of file (EOF) token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_eof(&self) -> bool {
        self.kind == EndOfFile
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{} ", self.pos)?;
        match self.kind {
            Int(i) => write!(f, "Int: {}", i),
            Real(r) => write!(f, "Real: {}", r),
            Str(ref s) => write!(f, "Str: \"{}\"", s),
            Identity(ref i) => write!(f, "Identity: \"{}\"", i),
            _ => write!(f, "{:?}", self.kind),
        }
    }
}

/// Performs a check on the input str `id` to see
/// whenever it is a keyword token or a name token
/// and then returns the coresponding `Token`.
pub fn lookup_identity(id: String) -> TokenKind {
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
