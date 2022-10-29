#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Identifier,
    Integer,
    Assign,
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

pub fn lookup_token(name: &str) -> TokenType {
    match name {
        "=" => TokenType::Assign,
        ";" => TokenType::Semicolon,
        "," => TokenType::Comma,
        "+" => TokenType::Plus,
        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        _ => TokenType::Identifier,
    }
}
