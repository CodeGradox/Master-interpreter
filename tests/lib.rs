extern crate interpreter;

use interpreter::lexer;
use interpreter::tokens::{Token, LexerError};
use interpreter::real::Real;

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
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
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_neverending_string() {
    let mut lexer = lexer::Lexer::new("\"This string never ends");
    let tokens = vec![Err(LexerError::NonTerminatingString), Ok(Token::EndOfFile)];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
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
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_string_escape() {
    let mut lexer = lexer::Lexer::new("\" \\\\ \\n \\t \\r \"");
    let tokens = vec![Ok(Token::Str(" \\\\ \\n \\t \\r ".to_string())), Ok(Token::EndOfFile)];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_string_illegal_newline() {
    let mut lexer = lexer::Lexer::new("\"\n\"\\");
    assert_eq!(Err(LexerError::StringEOL), lexer.next_token());
    assert_eq!(Err(LexerError::NonTerminatingString), lexer.next_token());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
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
#[cfg_attr(rustfmt, rustfmt_skip)]
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

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_int() {
    let a = Real::from(64);
    let b = Real::parse("64").unwrap();
    let c = Real::from(64.);
    assert!(a == b);
    assert!(a == c);
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_fraction() {
    let a = Real::parse("3.14").unwrap();
    let b = Real::from(3.14);
    assert!(a == b);
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_parse() {
    assert!(Real::parse("3.14").is_ok());
    assert!(Real::parse("3.").is_ok());
    assert!(Real::parse("3").is_ok());
    assert!(Real::parse(".").is_err());
    assert!(Real::parse("").is_err());
    assert!(Real::parse(".14").is_err());
}


#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_add() {
    let a = Real::from(97);
    let b = Real::from(3.0);
    assert!(a + b == Real::from(100));
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_sub() {
    let a = Real::from(3.5);
    let b = Real::from(3.0);
    assert!(a - b == Real::from(0.5));
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_mul() {
    let a = Real::from(2);
    let b = Real::from(2.0);
    assert!(a * b == Real::from(4));
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_div() {
    let a = Real::from(25);
    let b = Real::from(5.0);
    assert!(a / b == Real::from(5));
}
