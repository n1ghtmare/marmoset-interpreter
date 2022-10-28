use token::TokenType;

mod lexer;
mod token;

fn main() {
    println!("This is the marmoset language interpreter");
}

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
            token_type: TokenType::Lparen,
            literal: String::from("("),
        },
        token::Token {
            token_type: TokenType::Rparen,
            literal: String::from(")"),
        },
        token::Token {
            token_type: TokenType::Lbrace,
            literal: String::from("{"),
        },
        token::Token {
            token_type: TokenType::Rbrace,
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

    let mut lexer = lexer::Lexer::new(input);

    for result in expected_results {
        let tok = lexer.next_token();

        assert_eq!(tok.token_type, result.token_type);
        assert_eq!(tok.literal, result.literal);
    }
}

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
            token_type: TokenType::Ident,
            literal: String::from("five"),
        },
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Int,
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
            token_type: TokenType::Ident,
            literal: String::from("ten"),
        },
        token::Token {
            token_type: TokenType::Assign,
            literal: String::from("="),
        },
        token::Token {
            token_type: TokenType::Int,
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
            token_type: TokenType::Ident,
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
            token_type: TokenType::Lparen,
            literal: String::from("("),
        },
        token::Token {
            token_type: TokenType::Ident,
            literal: String::from("x"),
        },
        token::Token {
            token_type: TokenType::Comma,
            literal: String::from(","),
        },
        token::Token {
            token_type: TokenType::Ident,
            literal: String::from("y"),
        },
        token::Token {
            token_type: TokenType::Rparen,
            literal: String::from(")"),
        },
        token::Token {
            token_type: TokenType::Lbrace,
            literal: String::from("{"),
        },
        token::Token {
            token_type: TokenType::Ident,
            literal: String::from("x"),
        },
        token::Token {
            token_type: TokenType::Plus,
            literal: String::from("+"),
        },
        token::Token {
            token_type: TokenType::Ident,
            literal: String::from("y"),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
        token::Token {
            token_type: TokenType::Rbrace,
            literal: String::from("}"),
        },
        token::Token {
            token_type: TokenType::Semicolon,
            literal: String::from(";"),
        },
        token::Token {
            token_type: TokenType::Eof,
            literal: String::new(),
        },
    ];

    let mut lexer = lexer::Lexer::new(input);

    for result in expected_results {
        let tok = lexer.next_token();

        // println!("{:?}", tok.token_type);
        assert_eq!(tok.token_type, result.token_type);
        assert_eq!(tok.literal, result.literal);
    }
}
