extern crate interpreter;

use interpreter::lexer;
use interpreter::tokens::TokenKind;
use interpreter::real::Real;
use interpreter::error::Error;
use interpreter::error::ParseError;
use interpreter::error::ParseErrorKind;
use interpreter::source_pos::SourcePos;

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn next_token_test() {
    let mut lexer = lexer::Lexer::new("1 + 3 * 5");
    let tokens = vec![TokenKind::Int(1),
                      TokenKind::Plus,
                      TokenKind::Int(3),
                      TokenKind::Mul,
                      TokenKind::Int(5),
                      TokenKind::EndOfFile];

    for t in &tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, *t);
    }
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_neverending_string() {
    let mut lexer = lexer::Lexer::new("\"This string never ends");
    let err = Err(
        Error::ParseError(
            ParseError::new(ParseErrorKind::InfiniteString, SourcePos::new(1, 23))
        )
    );
    assert_eq!(err, lexer.next_token());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_string() {
    let mut lexer = lexer::Lexer::new("\"Hello World\"\
    \"\"
    \"33..89\"
    \"\\n\\r\\t\\n\"");
    let tokens = vec![TokenKind::Str("Hello World".to_string()),
                      TokenKind::Str("".to_string()),
                      TokenKind::Str("33..89".to_string()),
                      TokenKind::Str("\\n\\r\\t\\n".to_string()),
                      TokenKind::EndOfFile];

    for t in &tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, *t);
    }
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_string_escape() {
    let mut lexer = lexer::Lexer::new("\" \\\\ \\n \\t \\r \"");
    let tokens = vec![TokenKind::Str(" \\\\ \\n \\t \\r ".to_string()),
                      TokenKind::EndOfFile];

    for t in &tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, *t);
    }
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_string_illegal_newline() {
    let mut lexer = lexer::Lexer::new("\" \n");
    let err = Err(Error::ParseError(
                  ParseError::new(ParseErrorKind::StringEOL, SourcePos::new(1, 3))));
    assert_eq!(err, lexer.next_token());
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_error_illegals() {
    let mut lexer = lexer::Lexer::new("$%`^~💡");
    assert_eq!(lexer.next_token(), Err(Error::ParseError(
                  ParseError::new(ParseErrorKind::Illegal('$'), SourcePos::new(1, 1)))));
    assert_eq!(lexer.next_token(), Err(Error::ParseError(
                  ParseError::new(ParseErrorKind::Illegal('%'), SourcePos::new(1, 2)))));
    assert_eq!(lexer.next_token(), Err(Error::ParseError(
                  ParseError::new(ParseErrorKind::Illegal('`'), SourcePos::new(1, 3)))));
    assert_eq!(lexer.next_token(), Err(Error::ParseError(
                  ParseError::new(ParseErrorKind::Illegal('^'), SourcePos::new(1, 4)))));
    assert_eq!(lexer.next_token(), Err(Error::ParseError(
                  ParseError::new(ParseErrorKind::Illegal('~'), SourcePos::new(1, 5)))));
    assert_eq!(lexer.next_token(), Err(Error::ParseError(
                  ParseError::new(ParseErrorKind::Illegal('💡'), SourcePos::new(1, 6)))));
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_int_comp() {
    let a = Real::from(64);
    let b = Real::parse("64.0").unwrap();
    let c = Real::from(64.);
    assert!(a == b);
    assert!(a == c);
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_fraction_comp() {
    let a = Real::parse("3.14").unwrap();
    let b = Real::from(3.14);
    assert!(a == b);
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn test_real_parse() {
    assert!(Real::parse("3.14").is_ok());
    assert!(Real::parse("0.0").is_ok());
    assert!(Real::parse("1034.9999").is_ok());
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
