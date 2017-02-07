extern crate interpreter;

use interpreter::lexer;
use interpreter::tokens::Token;

const STRING_TEST: &'static str = include_str!("string_test.txt");

#[test]
fn next_token_test() {
    let mut lexer = lexer::Lexer::new("1 + 3 * 5");
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
    let mut lexer = lexer::Lexer::new("\"This string never ends");
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

#[test]
fn test_string_escape() {
    let string = "\" \\\\ \\n \\t \\r \"".to_string();
    let mut lexer = lexer::Lexer::new(&string);
    let tokens = vec![Token::Str(" \\\\ \\n \\t \\r ".to_string()), Token::EndOfFile];

    for t in &tokens {
        let token = lexer.next_token(); {
            assert_eq!(token, *t);
        }
    }
}