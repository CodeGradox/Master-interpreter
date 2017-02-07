use tokens::Token::*;
use std::borrow::Cow;

/// Represents a token returned by `Lexer::get_token`
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Types
    Num(i32),
    // Real(i32),
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

    // End of file
    EndOfFile,

    // Error tokens
    NonTerminatingString,
    StringEOL,
    Illegal(char),
    UnknownEscape(char),
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

    /// Is this an error token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_error(&self) -> bool {
        match *self {
              NonTerminatingString
            | StringEOL
            | Illegal(_)
            | UnknownEscape(_) => true,
            _ => false,
        }
    }

    /// Returns the error message for the error tokens
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn error_msg(&self) -> Cow<'static, str> {
        match *self {
            NonTerminatingString => Cow::Borrowed("error: nonterminated string"),
            StringEOL => Cow::Borrowed("error: found newline while scanning string"),
            Illegal(c) => Cow::Owned(format!("error: illegal character {}", c)),
            UnknownEscape(c) => {
                let esc: String = c.escape_default().collect();
                Cow::Owned(format!("error: unknown escape: '{}' in string literal", esc))
            }
            _ => Cow::Borrowed(""),
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
