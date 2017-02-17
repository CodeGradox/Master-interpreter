use tokens;
use tokens::{Token, LexerError, LexerResult};

use std::str::Chars;
use std::iter::Peekable;

/// The Lexical scanner.
/// It performs a lexical scanning of a string.
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: u32,
    column: u32,
    start: u32,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from a string slice.
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
            start: 1,
        }
    }

    /// Andvances the scanner and returns the next `char`.
    /// If the input is empty, it returns `None`.
    fn read_char(&mut self) -> Option<char> {
        self.column += 1;
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

    /// Reads and discards the next `char` from input
    /// and advances the iterator.
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
                self.line += 1;
                self.column = 0;
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
    fn read_identifier(&mut self, first: char) -> Token {
        let mut buf = String::new();
        buf.push(first);
        self.read_while(&mut buf, is_alphanumeric);
        tokens::lookup_identity(buf)
    }

    /// Reads a number from the input.
    /// Returns a `Result<Token, LexerError>` where `Token` is
    /// either an `Int` or a `Real`. The error happens when
    /// the parsing of the number fails.
    fn read_number(&mut self, first: char) -> LexerResult {
        let mut buf = String::new();
        buf.push(first);
        self.read_while(&mut buf, is_numeric);

        // The number can be followed by a decimal or a range
        if self.peek_char_eq('.') {
            // so we need to find out how many dots there are
            let count = self.input
                .clone()
                .take_while(|&x| x == '.')
                .take(2)
                .count();
            if count == 1 {
                buf.push(self.read_char().unwrap());
                self.read_while(&mut buf, is_numeric);
                return Ok(Token::Real(buf));
            }
            // else we just return the int
        }
        // This solution will not handle -2147483648 because
        // it does not know if the previous token was a `Minus`.
        match buf.parse() {
            Ok(val) => Ok(Token::Int(val)),
            Err(_) => Err(LexerError::IntLiteralTooLarge),
        }
    }

    /// Reads a string literal from the input.
    fn read_string(&mut self) -> LexerResult {
        let mut buf = String::new();

        // Loop until it finds a ".
        // Finding a newline or EOF results in an error.
        while let Some(&c) = self.peek_char() {
            if c == '\\' {
                buf.push(self.read_char().unwrap());
                match self.peek_char() {
                    Some(&p) => {
                        if !is_escape_char(p) {
                            return Err(LexerError::UnknownEscape(p));
                        }
                    }
                    None => break,
                }
            } else if c == '\n' {
                return Err(LexerError::StringEOL);
            } else if c == '"' {
                self.skip();
                return Ok(Token::Str(buf));
            }
            buf.push(self.read_char().unwrap());
        }
        Err(LexerError::NonTerminatingString)
    }

    /// Generates a `Token` from the characters read from the input.
    /// It traverses the input one `char` at the time and generates `Token`s.
    /// When the whole input has been scanned, the lexer will yield
    /// a `Token::EndOfFile` token. It returns an `LexerError` if it encounters
    /// and error while scanning the input.
    ///
    /// # Remarks
    /// Calling this method will advance the lexer.
    /// The lexer traverses the input only once.
    pub fn next_token(&mut self) -> LexerResult {
        self.skip_whitespace();
        self.start = self.column;

        if let Some(c) = self.read_char() {
            match c {
                '@' => Ok(Token::At),
                ',' => Ok(Token::Comma),
                ';' => Ok(Token::Semicolon),
                '{' => Ok(Token::LeftCurlyParam),
                '}' => Ok(Token::RightCurlyParam),
                '[' => Ok(Token::LeftSquareParam),
                ']' => Ok(Token::RightSquareParam),
                '(' => Ok(Token::LeftParam),
                ')' => Ok(Token::RightParam),
                '?' => Ok(Token::QuestionMark),
                '&' => Ok(Token::And),
                '|' => Ok(Token::Or),
                '!' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::NotEqual)
                    } else {
                        Ok(Token::Not)
                    }
                }
                '=' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::Equal)
                    } else {
                        Ok(Token::Assignment)
                    }
                }
                '+' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::PlusAssignment)
                    } else {
                        Ok(Token::Plus)
                    }
                }
                '-' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::MinusAssignment)
                    } else {
                        Ok(Token::Minus)
                    }
                }
                '*' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::MulAssignment)
                    } else {
                        Ok(Token::Mul)
                    }
                }
                '/' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::DivAssignment)
                    } else {
                        Ok(Token::Div)
                    }
                }
                '>' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::GreaterEqual)
                    } else {
                        Ok(Token::GreaterThan)
                    }
                }
                '<' => {
                    if self.peek_char_eq('=') {
                        self.skip();
                        Ok(Token::LessEqual)
                    } else {
                        Ok(Token::LessThan)
                    }
                }
                '.' => {
                    if self.peek_char_eq('.') {
                        self.skip();
                        if self.peek_char_eq('.') {
                            self.skip();
                            Ok(Token::InclusiveRange)
                        } else {
                            Ok(Token::ExclusiveRange)
                        }
                    } else {
                        Ok(Token::Dot)
                    }
                }
                '0'...'9' => self.read_number(c),
                '"' => self.read_string(),
                _ => {
                    if is_letter(c) {
                        Ok(self.read_identifier(c))
                    } else {
                        Err(LexerError::Illegal(c))
                    }
                }
            }
        } else {
            Ok(Token::EndOfFile)
        }
    }

    /// Returns the current line number.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the starting position of the last `Token`.
    pub fn pos(&self) -> u32 {
        self.start
    }
}

impl<'a> Iterator for Lexer<'a> {
    /// The type of the elements being iterated over.
    /// It returns a `Result`, where the `Ok` variant is a `Token`
    /// and the `Err`is an error encountered while scanning the input.
    /// The first `u32` is the current line number.
    /// The second `u32` is the starting position of the `Token`.
    type Item = (LexerResult, u32, u32);

    /// Advances the iterator and returns the next value.
    /// It returns `None` when the `Lexer` returns a `Token::EndOfFile` token.
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Token::EndOfFile) => None,
            token => Some((token, self.line(), self.pos())),
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

/// Returns true if `c` is an escape character
fn is_escape_char(c: char) -> bool {
    match c {
        '"' | 'n' | 't' | 'r' | '\\' => true,
        _ => false,
    }
}
