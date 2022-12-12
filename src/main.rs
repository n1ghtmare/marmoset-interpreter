mod ast;
mod lexer;
mod parser;
mod token;

use lexer::*;
use parser::*;
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
        Token::new(Integer, "1337"),
        Token::new(Semicolon, ";"),
    ];

    for result in expected_results {
        let tok = lexer.next_token().unwrap();
        println!("literal: {}", tok.literal);

        dbg!(tok.token_type, result.token_type);
        dbg!(tok.literal, result.literal);
    }
}

// The rust compiler is complaining that the function is not used anywhere other than the test
// The below flag will mark the functio to be ignored by the compiler
// From: https://stackoverflow.com/questions/32900809/how-to-suppress-function-is-never-used-warning-for-a-function-used-by-tests
#[cfg(test)]
use pretty_assertions::assert_eq;

use crate::ast::Node;

#[cfg(test)]
fn assert_expected_results(input: String, expected_results: Vec<Token>) {
    let lexer = lexer::Lexer::new(input);
    let tokens: Vec<Token> = lexer.into_iter().collect();

    assert_eq!(expected_results, tokens);
}

#[test]
fn test_equality_operators() {
    let input = String::from("!= ==");

    let expected_results: Vec<Token> = vec![Token::new(NotEqual, "!="), Token::new(Equal, "==")];

    assert_expected_results(input, expected_results);
}

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

// #[ignore]
#[test]
fn testing_if_statement() {
    let input = String::from("if (5 < 10)");

    let expected_results: Vec<Token> = vec![
        Token::new(If, "if"),
        Token::new(LeftParen, "("),
        Token::new(Integer, "5"),
        Token::new(LowerThan, "<"),
        Token::new(Integer, "10"),
        Token::new(RightParen, ")"),
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

// #[ignore]
#[test]
fn test_multiline_statements_with_keywords_extended() {
    let input = String::from(
        "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;",
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
        Token::new(Let, "let"),
        Token::new(Identifier, "result"),
        Token::new(Assignment, "="),
        Token::new(Identifier, "add"),
        Token::new(LeftParen, "("),
        Token::new(Identifier, "five"),
        Token::new(Comma, ","),
        Token::new(Identifier, "ten"),
        Token::new(RightParen, ")"),
        Token::new(Semicolon, ";"),
        Token::new(Bang, "!"),
        Token::new(Minus, "-"),
        Token::new(Slash, "/"),
        Token::new(Asterisk, "*"),
        Token::new(Integer, "5"),
        Token::new(Semicolon, ";"),
        Token::new(Integer, "5"),
        Token::new(LowerThan, "<"),
        Token::new(Integer, "10"),
        Token::new(GreaterThan, ">"),
        Token::new(Integer, "5"),
        Token::new(Semicolon, ";"),
        Token::new(If, "if"),
        Token::new(LeftParen, "("),
        Token::new(Integer, "5"),
        Token::new(LowerThan, "<"),
        Token::new(Integer, "10"),
        Token::new(RightParen, ")"),
        Token::new(LeftBrace, "{"),
        Token::new(Return, "return"),
        Token::new(True, "true"),
        Token::new(Semicolon, ";"),
        Token::new(RightBrace, "}"),
        Token::new(Else, "else"),
        Token::new(LeftBrace, "{"),
        Token::new(Return, "return"),
        Token::new(False, "false"),
        Token::new(Semicolon, ";"),
        Token::new(RightBrace, "}"),
        Token::new(Integer, "10"),
        Token::new(Equal, "=="),
        Token::new(Integer, "10"),
        Token::new(Semicolon, ";"),
        Token::new(Integer, "10"),
        Token::new(NotEqual, "!="),
        Token::new(Integer, "9"),
        Token::new(Semicolon, ";"),
    ];

    assert_expected_results(input, expected_results);
}

// #[ignore]
#[test]
fn test_let_statements_parsing() {
    let input = String::from(
        "let x = 5;
let y = 10;
let foobar = 838383;
",
    );

    let lexer = Lexer::new(input);

    let parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert!(program.is_some());

    // Is this the best way to run those asserts? First check if is_some()? then panick if it isn't? Doesn't sound right, do some reasearch!
    let program = program.expect("Failed to parse program");

    // TODO: We could probably write better assert for these
    assert_eq!(program.statements.len(), 3);
    assert_eq!(program.statements[0].token_literal(), "let");
}
