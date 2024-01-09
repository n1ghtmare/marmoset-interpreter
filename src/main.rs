pub mod lexer;
pub mod token;

use lexer::Lexer;
use token::{Token, TokenType};

fn main() {
    // TODO: We will use this to run the REPL
    println!("This is the marmoset language interpreter");

    let input = String::from("let marmoset = 1337;");
    let mut lexer = Lexer::new(input);

    let expected_results: Vec<Token> = vec![
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Identifier, "marmoset"),
        Token::new(TokenType::Assignment, "="),
        Token::new(TokenType::Integer, "1337"),
        Token::new(TokenType::Semicolon, ";"),
    ];

    for result in expected_results {
        let tok = lexer.next().unwrap();
        println!("literal: {}", tok.literal);

        dbg!(tok.token_type, result.token_type);
        dbg!(tok.literal, result.literal);
    }
}
