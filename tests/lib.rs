extern crate interpreter;

use interpreter::lexer;
use interpreter::tokens::Token;

const DATA: &'static str = include_str!("ex1.txt");

#[test]
fn next_token_test() {
    let mut lexer = lexer::Lexer::new(&DATA);
    let tokens = vec![
        Token::Num(1),
        Token::Plus,
        Token::Num(3),
        Token::Mul,
        Token::Num(5),
        Token::EndOfFile,
    ];

    for t in &tokens {
        let token = lexer.next_token();
        assert_eq!(token, *t);
    }
}