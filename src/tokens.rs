#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Types
    Num(i32),
    // Real(i32),
    Str(String),
    Nil,

    // Identifier
    Ident(String),

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
    At,
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
    Dot,
    Comma,

    // End of file
    EndOfFile,

    // Error in scannign and illegal characters
    Error(String),
    Illegal(String),
}

impl Token {
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
}

pub fn lookup_identity(id: &str) -> Token {
    match id {
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
        _ => Token::Ident(id.to_string()),
    }
}

pub fn illegal_token(msg: &str) -> Token {
    Token::Illegal(msg.to_string())
}

pub fn error(msg: &str) -> Token {
    Token::Error(msg.to_string())
}
