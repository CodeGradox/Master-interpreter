
use tokens::{Token, TokenKind};
use lexer::Lexer;
use error;
use source_pos::SourcePos;

use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { lexer: Lexer::new(input).peekable() }
    }

    pub fn run(&mut self) -> error::Result<()> {
        // while self.peek().is_some() {
        //     self.parse_expr()?;
        // }
        // Ok(())
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> error::Result<()> {
        self.parse_atom()
    }

    // primitive or literal
    fn parse_atom(&mut self) -> error::Result<()> {
        if let Some(next) = self.next() {
            let token = next?;
            if token.is_literal() {
                println!("{:?}", token);
                return Ok(());
            } else if token.is_identity() {
                println!("{:?}", token);
                return Ok(());
            } else {
                let pos = SourcePos::new(token.line(), token.col());
                let msg = format!("{}", token);
                let err = error::ParseError::new(
                    error::ParseErrorKind::ExpectedAtom(msg), pos);
                return Err(error::Error::from(err));
            }

        }

        let pos = SourcePos::new(0, 0);
        let err = error::ParseError::new(
            error::ParseErrorKind::EndOfFile("expected primitive".to_string()), pos);
        Err(error::Error::from(err))
    }

    fn next(&mut self) -> Option<error::Result<Token>> {
        self.lexer.next()
    }

    fn peek(&mut self) -> Option<&error::Result<Token>> {
        self.lexer.peek()
    }

    // fn accept<F>(&mut self, cond: F) -> error::Result<()>
    //     where F: Fn(TokenKind) -> bool
    // {
    //     if cond() {
    //         Ok(())
    //     } else {
    //         Err(error::ParseErrorKind::)
    //     }
    // }

    // fn ret_err() -> error::Result<()> {
    //     Err(

    //     )
    // }
}
