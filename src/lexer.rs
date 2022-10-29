use crate::token::{lookup_token, Token, TokenType};

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

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch.unwrap()) {
            self.read_char();
        }
        // TODO: Investigate the below - strings in rust are UTF-8
        return self.input[position..self.position].to_string();
    }

    fn is_letter(&mut self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_digit(&mut self, ch: char) -> bool {
        ch.is_numeric()
    }

    fn is_known_single(&mut self, ch: char) -> bool {
        let known_chars: Vec<char> = vec!['=', ';', '(', ')', ',', '+', '{', '}'];
        known_chars.contains(&ch)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.map(char::is_whitespace).unwrap_or_default() {
            self.read_char();
        }
    }

    fn process_char(&mut self, ch: char) -> Token {
        if self.is_known_single(ch) {
            self.read_char();
            let literal = ch.to_string();
            Token {
                token_type: lookup_token(literal.as_str()),
                literal,
            }
        } else if self.is_letter(ch) {
            let literal = self.read_identifier();
            Token {
                token_type: lookup_token(literal.as_str()),
                literal,
            }
        } else if self.is_digit(ch) {
            Token {
                token_type: TokenType::Integer,
                literal: self.read_number(),
            }
        } else {
            Token {
                token_type: TokenType::Illegal,
                literal: String::from("illegal"),
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let result: Token = match self.ch {
            Some(ch) => self.process_char(ch),
            None => Token {
                token_type: TokenType::Illegal,
                literal: String::from("illegal"),
            },
        };

        return result;
    }
}
