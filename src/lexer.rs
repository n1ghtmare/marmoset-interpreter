use crate::token::{lookup_ident, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();

        return lexer;
    }

    // read characters one by one if we reach the end ? return None
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch.unwrap()) {
            self.read_char();
        }
        // TODO: Investigate the below - strings in rust are UTF-8
        return self.input[position..self.position].to_string();
    }

    fn is_letter(&mut self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    pub fn next_token(&mut self) -> Token {
        let ch = self.ch.unwrap();

        let token = match ch {
            '=' => Token {
                token_type: TokenType::Assign,
                literal: ch.to_string(),
            },
            ';' => Token {
                token_type: TokenType::Semicolon,
                literal: ch.to_string(),
            },
            '(' => Token {
                token_type: TokenType::Lparen,
                literal: ch.to_string(),
            },
            ')' => Token {
                token_type: TokenType::Rparen,
                literal: ch.to_string(),
            },
            ',' => Token {
                token_type: TokenType::Comma,
                literal: ch.to_string(),
            },
            '+' => Token {
                token_type: TokenType::Plus,
                literal: ch.to_string(),
            },
            '{' => Token {
                token_type: TokenType::Lbrace,
                literal: ch.to_string(),
            },
            '}' => Token {
                token_type: TokenType::Rbrace,
                literal: ch.to_string(),
            },
            _ => {
                if self.is_letter(ch) {
                    let literal = self.read_identifier();
                    Token {
                        token_type: lookup_ident(literal.as_str()),
                        literal,
                    }
                } else {
                    Token {
                        token_type: TokenType::Illegal,
                        literal: ch.to_string(),
                    }
                }
            }
        };

        self.read_char();
        return token;
    }
}
