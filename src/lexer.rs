#![allow(dead_code)]

use tokens;
use tokens::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input: input.chars().peekable() }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_char_eq(&mut self, other: &char) -> bool {
        match self.peek_char() {
            Some(&c) => c == *other,
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    // fn skip_char(&mut self) {
    //     self.read_char();
    // }

    fn read_while<F>(&mut self, first: char, func: F) -> String
        where F: Fn(char) -> bool {
        let mut buf = String::new();
        buf.push(first);

        while let Some(&c) = self.peek_char() {
            if !func(c) {
                break;
            }
            buf.push(self.read_char().unwrap());
        }
        buf
    }

    fn read_identifier(&mut self, first: char) -> String {
        self.read_while(first, is_alphanumeric)
    }

    fn read_number(&mut self, first: char) -> String {
        self.read_while(first, is_numeric)
    }

    fn read_real(&mut self, first: char) -> i32 {
        unimplemented!();
    }

    fn read_string(&mut self, first: char) -> String {
        self.read_while(first, |c| c != '"')
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('"') => {
                let c = match self.read_char() {
                    Some(c) => c,
                    None => return Token::End,
                };
                let string = self.read_string(c);

                match self.read_char() {
                    Some(_) => Token::Str(string),
                    None => Token::End,
                }
            },
            Some(c @ _) => {
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
