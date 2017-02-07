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

    // Error in scanning strings and illegal characters
    StringError,
    Illegal(char),
    IllegalEscape(char),
}

impl Token {
    /// Is this a keyword token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_keyword(&self) -> bool {
        match *self {
            Token::At
            | Token::Function
            | Token::True
            | Token::False
            | Token::If
            | Token::Else
            | Token::While
            | Token::For
            | Token::Break
            | Token::Return
            | Token::QuestionMark => true,
            _ => false,
        }
    }

    /// Is this an assignment token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_assignment(&self) -> bool {
        match *self {
            Token::Assignment
            | Token::PlusAssignment
            | Token::MinusAssignment
            | Token::MulAssignment
            | Token::DivAssignment => true,
            _ => false,
        }
    }

    /// Is this an arithmetic  token?
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn is_arithmetic(&self) -> bool {
        match *self {
            Token::Plus
            | Token::Minus
            | Token::Mul
            | Token::Div => true,
            _ => false,
        }
    }
}

/// Performs a check on the input str `id` to see
/// whenever it is a keyword token or a name token
/// and then returns the coresponding `Token`.
pub fn lookup_identity(id: String) -> Token {
    match id.as_str() {
        "fn" => Token::Function,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,
        "for" => Token::For,
        "break" => Token::Break,
        "return" => Token::Return,
        "nil" => Token::Nil,
        _ => Token::Identity(id),
    }
}
