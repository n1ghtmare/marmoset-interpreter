use token::{Token, TokenType};

mod lexer;
mod token;

fn main() {
    // TODO: We will use this to run the REPL
    println!("This is the marmoset language interpreter");

    let input = String::from("let marmoset = 1337;");
    let mut lexer = lexer::Lexer::new(input);

    let expected_results: Vec<Token> = vec![
        Token {
            token_type: TokenType::Let,
            literal: String::from("let"),
        },
        Token {
            token_type: TokenType::Identifier,
            literal: String::from("testing"),
        },
        Token {
            token_type: TokenType::Assignment,
            literal: String::from("="),
        },
        Token {
            token_type: TokenType::Integer,
            literal: String::from("5"),
        },
        Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
    ];

    for result in expected_results {
        let tok = lexer.next_token();
        println!("literal: {}", tok.literal);

        dbg!(tok.token_type, result.token_type);
        dbg!(tok.literal, result.literal);
    }
}

// The rust compiler is complaining that the function is not used anywhere other than the test
// The below flag will mark the functio to be ignored by the compiler
// From: https://stackoverflow.com/questions/32900809/how-to-suppress-function-is-never-used-warning-for-a-function-used-by-tests
#[cfg(test)]
fn assert_expected_results(input: String, expected_results: Vec<Token>) {
    let mut lexer = lexer::Lexer::new(input);

    for result in expected_results {
        let tok = lexer.next_token();
        // println!("literal: {}", tok.literal);

        assert_eq!(tok.token_type, result.token_type);
        assert_eq!(tok.literal, result.literal);
    }
}

// #[ignore]
#[test]
fn test_next_token_simple() {
    let input = String::from("let testing =5;");

    let expected_results: Vec<Token> = vec![
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Identifier, "testing"),
        Token::new(TokenType::Assignment, "="),
        Token::new(TokenType::Integer, "5"),
        Token::new(TokenType::Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}

// #[ignore]
#[test]
fn test_next_token_1() {
    let input = String::from("=+(){},;");

    let expected_results: Vec<Token> = vec![
        Token::new(TokenType::Assignment, "="),
        Token::new(TokenType::Plus, "+"),
        Token::new(TokenType::LeftParen, "("),
        Token::new(TokenType::RightParen, ")"),
        Token::new(TokenType::LeftBrace, "{"),
        Token::new(TokenType::RightBrace, "}"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}

// #[ignore]
#[test]
fn test_next_token_2() {
    let input = String::from(
        "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};",
    );

    let expected_results: Vec<Token> = vec![
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Identifier, "five"),
        Token::new(TokenType::Assignment, "="),
        Token::new(TokenType::Integer, "5"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Identifier, "ten"),
        Token::new(TokenType::Assignment, "="),
        Token::new(TokenType::Integer, "10"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Identifier, "add"),
        Token::new(TokenType::Assignment, "="),
        Token::new(TokenType::Function, "fn"),
        Token::new(TokenType::LeftParen, "("),
        Token::new(TokenType::Identifier, "x"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Identifier, "y"),
        Token::new(TokenType::RightParen, ")"),
        Token::new(TokenType::LeftBrace, "{"),
        Token::new(TokenType::Identifier, "x"),
        Token::new(TokenType::Plus, "+"),
        Token::new(TokenType::Identifier, "y"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::RightBrace, "}"),
        Token::new(TokenType::Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}
