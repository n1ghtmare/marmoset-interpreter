#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum TokenType {
    Identifier,
    Integer,
    Assignment,
    Plus,
    Comma,
    Minus,
    Bang,
    Slash,
    Asterisk,
    LowerThan,
    GreaterThan,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,
    Illegal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
            "-" => Minus,
            "!" => Bang,
            "/" => Slash,
            "*" => Asterisk,
            "<" => LowerThan,
            ">" => GreaterThan,
            "==" => Equal,
            "!=" => NotEqual,
            "fn" => Function,
            "let" => Let,
            "true" => True,
            "false" => False,
            "if" => If,
            "else" => Else,
            "return" => Return,
            _ => Identifier,
        }
    }
}
