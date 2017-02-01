use tokens;
use tokens::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line_num: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line_num: 1,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, other: char) -> bool {
        match self.peek_char() {
            Some(&c) => c == other,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c == '\n' {
                self.line_num += 1;
            }
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn skip(&mut self) {
        self.read_char();
    }

    fn read_while<F>(&mut self, buf: &mut String, func: F)
        where F: Fn(char) -> bool {
        while let Some(&c) = self.peek_char() {
            if !func(c) {
                break;
            }
            buf.push(self.read_char().unwrap());
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut buf = String::new();
        buf.push(first);
        self.read_while(&mut buf, is_alphanumeric);
        buf
    }

    fn read_number(&mut self, first: char) -> String {
        let mut buf = String::new();
        buf.push(first);
        self.read_while(&mut buf, is_numeric);
        buf
    }

    fn read_real(&mut self, first: char) -> i32 {
        unimplemented!();
    }

    fn read_string(&mut self) -> String {
        let mut buf = String::new();
        self.read_while(&mut buf, |c| c != '"');
        buf
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('@') => Token::At,
            Some('{') => Token::LeftCurlyParam,
            Some('}') => Token::RightCurlyParam,
            Some('[') => Token::LeftSquareParam,
            Some(']') => Token::RightSquareParam,
            Some('(') => Token::LeftParam,
            Some(')') => Token::RightParam,
            Some('?') => Token::QuestionMark,
            Some('&') => Token::And,
            Some('|') => Token::Or,
            Some('!') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            },
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::Equal
                } else {
                    Token::Assignment
                }
            },
            Some('+') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::PlusAssignment
                } else {
                    Token::Plus
                }
            },
            Some('-') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::MinusAssignment
                } else {
                    Token::Minus
                }
            },
            Some('*') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::MulAssignment
                } else {
                    Token::Mul
                }
            },
            Some('/') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::DivAssignment
                } else {
                    Token::Div
                }
            },
            Some('>') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            },
            Some('<') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            },
            Some('"') => {
                let string = self.read_string();
                match self.read_char() {
                    Some(_) => Token::Str(string),
                    None => tokens::error("No end of string!"),
                }
            },
            Some(c) => {
                if is_letter(c) {
                    let ident = self.read_identifier(c);
                    tokens::lookup_identity(&ident)
                } else if is_numeric(c) {
                    let num = self.read_number(c);
                    Token::Num(num.parse().unwrap())
                } else {
                    Token::Illegal
                }
            },
            None => Token::End
        }
    }

    pub fn line_number(&self) -> u32 {
        self.line_num
    }
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_numeric(c: char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

fn is_alphanumeric(c: char) -> bool {
    is_letter(c) || is_numeric(c)
}
