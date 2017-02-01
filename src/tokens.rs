#[derive(Debug, PartialEq)]
pub enum Token {
    // Types
    Num(i32),
    Real(i32),
    Str(String),
    Nil,

    // Identifier
    Ident(String),

    // Assignment
    Assignment,
    AddAssignment,
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

    // Misc
    Semicolon,
    Dot,
    Comma,
    QuestionMark,
    ExclamationMark,

    // End of file
    End,
    Illegal,
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
        _ => Token::Ident(id.to_owned()),
    }
}
