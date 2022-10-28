use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    Illegal,
}

// type TokenType = String;

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub fn lookup_ident(name: &str) -> TokenType {
    match name {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        _ => TokenType::Ident,
    }
}
