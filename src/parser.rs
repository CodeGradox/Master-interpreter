use tokens::Token;
use lexer::Lexer;
use error;
use error::Error;

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

        Ok(())
    }

    fn parse_expr(&mut self) -> error::Result<()> {

        Ok(())
    }

    fn parse_primitive(&mut self) -> error::Result<()> {
        // if let Some(next) = self.peek() {
        //     let &(ref token, ref line, ref pos) = next;
        //     match token {
        //         &Ok(Token::Int(_)) => Ok(()),
        //         _ => Err(Error::TempParseErr)
        //     }
        //     // match *next {
        //     //     (Ok(Token::Int(n)), _, _) => Ok(()),
        //     //     _ => Err(Error::TempParseErr)
        //     // }
        // } else {
        //     // Expected token, found end of file
        //     Err((Error::TempParseErr))
        // }
        Ok(())
    }

    // fn next(&mut self) -> Option<LexerItem> {
    //     self.lexer.next()
    // }

    // fn next_token(&mut self) -> Option<error::Result<Token>> {
    //     self.next().map(|x| x.0)
    // }

    // fn peek(&mut self) -> Option<&LexerItem> {
    //     self.lexer.peek()
    // }

    // fn peek_token(&mut self) -> Option<&error::Result<Token>> {
    //     self.peek().map(|x| &x.0)
    // }
}
