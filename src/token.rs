#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Identifier,
    Integer,
    Assignment,
    Plus,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    Let,
    Illegal,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

use TokenType::*;

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn lookup_token(name: &str) -> TokenType {
        match name {
            "=" => Assignment,
            ";" => Semicolon,
            "," => Comma,
            "+" => Plus,
            "(" => LeftParen,
            ")" => RightParen,
            "{" => LeftBrace,
            "}" => RightBrace,
            "fn" => Function,
            "let" => Let,
            _ => Identifier,
        }
    }
}
