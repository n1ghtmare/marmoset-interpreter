mod lexer;
mod token;

use token::*;
use TokenType::*;

fn main() {
    // TODO: We will use this to run the REPL
    println!("This is the marmoset language interpreter");

    let input = String::from("let marmoset = 1337;");
    let mut lexer = lexer::Lexer::new(input);

    let expected_results: Vec<Token> = vec![
        Token::new(Let, "let"),
        Token::new(Identifier, "testing"),
        Token::new(Assignment, "="),
        Token::new(Integer, "5"),
        Token::new(Semicolon, ";"),
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
fn test_single_variable_binding() {
    let input = String::from("let _testing_a = 5;");

    let expected_results: Vec<Token> = vec![
        Token::new(Let, "let"),
        Token::new(Identifier, "_testing_a"),
        Token::new(Assignment, "="),
        Token::new(Integer, "5"),
        Token::new(Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}

#[test]
fn test_multi_byte_chars() {
    let input = String::from("let 猿猿 = 5;");

    let expected_results: Vec<Token> = vec![
        Token::new(Let, "let"),
        Token::new(Identifier, "猿猿"),
        Token::new(Assignment, "="),
        Token::new(Integer, "5"),
        Token::new(Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}

// #[ignore]
#[test]
fn test_single_character_tokens() {
    let input = String::from("=+(){},;");

    let expected_results: Vec<Token> = vec![
        Token::new(Assignment, "="),
        Token::new(Plus, "+"),
        Token::new(LeftParen, "("),
        Token::new(RightParen, ")"),
        Token::new(LeftBrace, "{"),
        Token::new(RightBrace, "}"),
        Token::new(Comma, ","),
        Token::new(Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}

// #[ignore]
#[test]
fn test_multiline_statements_with_keywords() {
    let input = String::from(
        "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};",
    );

    let expected_results: Vec<Token> = vec![
        Token::new(Let, "let"),
        Token::new(Identifier, "five"),
        Token::new(Assignment, "="),
        Token::new(Integer, "5"),
        Token::new(Semicolon, ";"),
        Token::new(Let, "let"),
        Token::new(Identifier, "ten"),
        Token::new(Assignment, "="),
        Token::new(Integer, "10"),
        Token::new(Semicolon, ";"),
        Token::new(Let, "let"),
        Token::new(Identifier, "add"),
        Token::new(Assignment, "="),
        Token::new(Function, "fn"),
        Token::new(LeftParen, "("),
        Token::new(Identifier, "x"),
        Token::new(Comma, ","),
        Token::new(Identifier, "y"),
        Token::new(RightParen, ")"),
        Token::new(LeftBrace, "{"),
        Token::new(Identifier, "x"),
        Token::new(Plus, "+"),
        Token::new(Identifier, "y"),
        Token::new(Semicolon, ";"),
        Token::new(RightBrace, "}"),
        Token::new(Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}
