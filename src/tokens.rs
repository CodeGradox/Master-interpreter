use tokens::Token::*;
use tokens::LexerError::*;

pub type LexerResult = Result<Token, LexerError>;

/// Represents a valid token returned by `Lexer::get_token`
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Types
    Int(i32),
    Real(String),
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

/// Represents a error encountered during the lexical analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    NonTerminatingString,
    StringEOL,
    Illegal(char),
    UnknownEscape(char),
    IntLiteralTooLarge,
}

impl LexerError {
    /// Prints out the error and it's location.
    pub fn print_err(&self, line: u32, pos: u32) {
        print!("error! line: {} col {}\n\t", line, pos);
        match *self {
            NonTerminatingString => println!("nonterminating string, found end of file"),
            StringEOL => println!("nonterminating string, found newline"),
            Illegal(c) => println!("found illegal token {}", c),
            UnknownEscape(c) => {
                let esc: String = c.escape_default().collect();
                println!("unknown escape code {}", esc);
            }
            IntLiteralTooLarge => println!("int literal too large"),
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
