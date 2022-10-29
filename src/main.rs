use token::TokenType;

mod lexer;
mod token;

fn main() {
    // TODO: We will use this to run the REPL

    println!("This is the marmoset language interpreter");

    let input = String::from("let marmoset = 1337;");
    let mut lexer = lexer::Lexer::new(input);

    let expected_results: Vec<token::Token> = vec![
        token::Token {
            token_type: TokenType::Let,
            literal: String::from("let"),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("testing"),
        },
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Integer,
            literal: String::from("5"),
        },
        token::Token {
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
fn assert_expected_results(input: String, expected_results: Vec<token::Token>) {
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

    let expected_results: Vec<token::Token> = vec![
        token::Token {
            token_type: TokenType::Let,
            literal: String::from("let"),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("testing"),
        },
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Integer,
            literal: String::from("5"),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
    ];

    assert_expected_results(input, expected_results);
}

// #[ignore]
#[test]
fn test_next_token_1() {
    let input = String::from("=+(){},;");

    let expected_results: Vec<token::Token> = vec![
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Plus,
            literal: String::from("+"),
        },
        token::Token {
            token_type: TokenType::LeftParen,
            literal: String::from("("),
        },
        token::Token {
            token_type: TokenType::RightParen,
            literal: String::from(")"),
        },
        token::Token {
            token_type: TokenType::LeftBrace,
            literal: String::from("{"),
        },
        token::Token {
            token_type: TokenType::RightBrace,
            literal: String::from("}"),
        },
        token::Token {
            token_type: TokenType::Comma,
            literal: String::from(","),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
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

    let expected_results: Vec<token::Token> = vec![
        token::Token {
            token_type: TokenType::Let,
            literal: String::from("let"),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("five"),
        },
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Integer,
            literal: String::from("5"),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
        token::Token {
            token_type: TokenType::Let,
            literal: String::from("let"),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("ten"),
        },
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Integer,
            literal: String::from("10"),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
        token::Token {
            token_type: TokenType::Let,
            literal: String::from("let"),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("add"),
        },
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Function,
            literal: String::from("fn"),
        },
        token::Token {
            token_type: TokenType::LeftParen,
            literal: String::from("("),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("x"),
        },
        token::Token {
            token_type: TokenType::Comma,
            literal: String::from(","),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("y"),
        },
        token::Token {
            token_type: TokenType::RightParen,
            literal: String::from(")"),
        },
        token::Token {
            token_type: TokenType::LeftBrace,
            literal: String::from("{"),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("x"),
        },
        token::Token {
            token_type: TokenType::Plus,
            literal: String::from("+"),
        },
        token::Token {
            token_type: TokenType::Identifier,
            literal: String::from("y"),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
        token::Token {
            token_type: TokenType::RightBrace,
            literal: String::from("}"),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
    ];

    assert_expected_results(input, expected_results);
}
