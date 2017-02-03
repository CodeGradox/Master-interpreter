extern crate interpreter;

use interpreter::lexer;
use interpreter::tokens::Token;

const DATA: &'static str = include_str!("ex1.txt");
const STRING_TEST: &'static str = include_str!("string_test.txt");

static NONSTOP_STR: &'static str = "\"This string never ends";

#[test]
fn next_token_test() {
    let mut lexer = lexer::Lexer::new(DATA);
    let tokens = vec![Token::Num(1),
                      Token::Plus,
                      Token::Num(3),
                      Token::Mul,
                      Token::Num(5),
                      Token::EndOfFile];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
fn test_neverending_string() {
    let mut lexer = lexer::Lexer::new(NONSTOP_STR);
    let tokens = vec![Token::StringError, Token::EndOfFile];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}

#[test]
fn test_string() {
    let mut lexer = lexer::Lexer::new(STRING_TEST);
    let tokens = vec![Token::Str("Hello World".to_string()),
                      Token::Str("".to_string()),
                      Token::Str("33..89".to_string()),
                      Token::Str("\\n\\r\\t\\n".to_string()),
                      Token::EndOfFile];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}