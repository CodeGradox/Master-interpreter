extern crate interpreter;

use interpreter::lexer;
use interpreter::tokens::{Token, LexerError};

#[test]
fn next_token_test() {
    let mut lexer = lexer::Lexer::new("1 + 3 * 5");
    let tokens = vec![Ok(Token::Int(1)),
                      Ok(Token::Plus),
                      Ok(Token::Int(3)),
                      Ok(Token::Mul),
                      Ok(Token::Int(5)),
                      Ok(Token::EndOfFile)];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
fn test_neverending_string() {
    let mut lexer = lexer::Lexer::new("\"This string never ends");
    let tokens = vec![Err(LexerError::NonTerminatingString), Ok(Token::EndOfFile)];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
fn test_string() {
    let mut lexer = lexer::Lexer::new("\"Hello World\"\
    \"\"
    \"33..89\"
    \"\\n\\r\\t\\n\"");
    let tokens = vec![Ok(Token::Str("Hello World".to_string())),
                      Ok(Token::Str("".to_string())),
                      Ok(Token::Str("33..89".to_string())),
                      Ok(Token::Str("\\n\\r\\t\\n".to_string())),
                      Ok(Token::EndOfFile)];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
fn test_string_escape() {
    let mut lexer = lexer::Lexer::new("\" \\\\ \\n \\t \\r \"");
    let tokens = vec![Ok(Token::Str(" \\\\ \\n \\t \\r ".to_string())), Ok(Token::EndOfFile)];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
fn test_string_illegal_newline() {
    let mut lexer = lexer::Lexer::new("\"\n\"\\");
    assert_eq!(Err(LexerError::StringEOL), lexer.next_token());
    assert_eq!(Err(LexerError::NonTerminatingString), lexer.next_token());
}

#[test]
fn test_error_tokens() {
    let mut lexer = lexer::Lexer::new("$%`^~");
    let tokens = vec![Err(LexerError::Illegal('$')),
                      Err(LexerError::Illegal('%')),
                      Err(LexerError::Illegal('`')),
                      Err(LexerError::Illegal('^')),
                      Err(LexerError::Illegal('~')),
                      Ok(Token::EndOfFile)];
    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
fn test_error_messages() {
    let mut lexer = lexer::Lexer::new("\"\n\
    💡 \
    \"\\x \
    \"");
    let tokens = vec![Err(LexerError::StringEOL),
                      Err(LexerError::Illegal('💡')),
                      Err(LexerError::UnknownEscape('x')),
                      Ok(Token::Identity("x".to_owned())), // The lexer does not consume the illegal escape
                      Err(LexerError::NonTerminatingString),
                      Ok(Token::EndOfFile)];
    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}