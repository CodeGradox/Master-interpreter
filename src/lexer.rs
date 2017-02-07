use tokens;
use tokens::Token;

use std::str::Chars;
use std::iter::Peekable;

/// The Lexical scanner.
/// It performs a lexical scanning of a string.
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line_num: u32,
    char_pos: u32,
    token_pos: u32,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from a string slice.
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line_num: 1,
            char_pos: 1,
            token_pos: 1,
        }
    }

    /// Andvances the scanner and returns the next `char`.
    /// If the input is empty, it returns `None`.
    fn read_char(&mut self) -> Option<char> {
        self.char_pos += 1;
        self.input.next()
    }

    /// Peeks at the next `char` from the input.
    /// It will advance the lexer if this function has not been called
    /// after a `read_char`.
    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Peeks at the next `char` and compares it to `other`
    /// and returns `true` if they are equal
    fn peek_char_eq(&mut self, other: char) -> bool {
        match self.peek_char() {
            Some(&c) => c == other,
            None => false,
        }
    }

    /// Reads the next `char` from input and discards it.
    fn skip(&mut self) {
        self.read_char();
    }

    /// Skips all `char`s until it finds a newline (`\n`)
    /// or until the end of file is reached.
    fn skip_line(&mut self) {
        while !self.peek_char_eq('\n') {
            self.skip();
        }
    }

    /// Skips all whitespace `char`s and comment blocks.
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c == '\n' {
                self.line_num += 1;
                self.char_pos = 0;
            } else if c == '#' {
                self.skip_line();
                continue;
            }
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    /// Helper function for reading multiple `char`s from the input based
    /// on a predicate.
    /// It will read each input into a buffer `String` while the
    /// predicate returns true.
    /// After a false is returned, `read_while()`'s job is finished
    /// and buffer is returned.
    fn read_while<F>(&mut self, buf: &mut String, func: F)
        where F: Fn(char) -> bool
    {
        while let Some(&c) = self.peek_char() {
            if !func(c) {
                break;
            }
            buf.push(self.read_char().unwrap());
        }
    }

    /// Reads an identifier string from the input.
    fn read_identifier(&mut self, first: char) -> String {
        let mut buf = String::new();
        buf.push(first);
        self.read_while(&mut buf, is_alphanumeric);
        buf
    }

    /// Reads a number from the input.
    fn read_number(&mut self, first: char) -> String {
        let mut buf = String::new();
        buf.push(first);
        self.read_while(&mut buf, is_numeric);
        buf
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn read_real(&mut self, first: char) -> i32 {
        unimplemented!();
    }

    /// Reads a string literal from the input.
    fn read_string(&mut self) -> Token {
        let mut buf = String::new();
        while let Some(&c) = self.peek_char() {
            if c == '\\' {
                buf.push(self.read_char().unwrap());
                match self.peek_char() {
                    Some(&p) =>
                        if !is_escape_char(p) {
                            return Token::UnknownEscape(p);
                        },
                    None => break,
                }
            } else if c == '\n' {
                return Token::StringEOL;
            } else if c == '"' {
                self.skip();
                return Token::Str(buf);
            }
            buf.push(self.read_char().unwrap());
        }
        Token::NonTerminatingString
    }

    /// Generates a `Token` from the characters read from the input.
    /// It traverses the input one `char` at the time and generates `Token`s.
    /// When the whole input has been scanned, the lexer will yield
    /// a `Token::EndOfFile` token.
    ///
    /// # Remarks
    /// Calling this method will advance the lexer.
    /// The lexer traverses the input only once.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.token_pos = self.char_pos;

        match self.read_char() {
            Some('@') => Token::At,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
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
            }
            Some('=') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::Equal
                } else {
                    Token::Assignment
                }
            }
            Some('+') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::PlusAssignment
                } else {
                    Token::Plus
                }
            }
            Some('-') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::MinusAssignment
                } else {
                    Token::Minus
                }
            }
            Some('*') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::MulAssignment
                } else {
                    Token::Mul
                }
            }
            Some('/') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::DivAssignment
                } else {
                    Token::Div
                }
            }
            Some('>') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            }
            Some('<') => {
                if self.peek_char_eq('=') {
                    self.skip();
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            }
            Some('.') => {
                if self.peek_char_eq('.') {
                    self.skip();
                    if self.peek_char_eq('.') {
                        self.skip();
                        Token::InclusiveRange
                    } else {
                        Token::ExclusiveRange
                    }
                } else {
                    Token::Dot
                }
            }
            Some('"') => self.read_string(),
            Some(c) => {
                if is_letter(c) {
                    let ident = self.read_identifier(c);
                    tokens::lookup_identity(ident)
                } else if is_numeric(c) {
                    let num = self.read_number(c);
                    Token::Num(num.parse().unwrap())
                } else {
                    Token::Illegal(c)
                }
            }
            None => Token::EndOfFile,
        }
    }

    /// Returns the current line number.
    ///
    /// It returns `1` if `next_token()` hasn't been called.
    pub fn line_number(&self) -> u32 {
        self.line_num
    }

    /// Returns the starting position of the last generated `Token`.
    ///
    /// It returns `1` if `next_token()` hasn't been called.
    pub fn current_token_pos(&self) -> u32 {
        self.token_pos
    }
}

impl<'a> Iterator for Lexer<'a> {
    /// The type of the elements being iterated over.
    /// `Token` is the current token.
    /// The first `u32` is the current line number.
    /// The second `u32` is the starting position of the `Token`.
    type Item = (Token, u32, u32);

    /// Advances the iterator and returns the next value.
    /// It returns `None` when the `Lexer` returns a `Token::EndOfFile` token.
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::EndOfFile => None,
            token => Some((token, self.line_number(), self.current_token_pos())),
        }
    }
}

/// Checks if `c` is a letter or a underscore (`_`).
fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// Checks if `c` is a number.
fn is_numeric(c: char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

/// Chekcs is `c` is a number, letter or a underscore (`_`).
fn is_alphanumeric(c: char) -> bool {
    is_letter(c) || is_numeric(c)
}

/// Returns true if c is an escape character
fn is_escape_char(c: char) -> bool {
    match c {
        '"' | 'n' | 't' | 'r' | '\\' => true,
        _ => false,
    }
}
