use crate::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
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
        let mut len = 0;
        while self.ch.is_some() && self.is_letter(self.ch.unwrap()) {
            len += 1;
            self.read_char();
        }

        return self
            .input
            .chars()
            .skip(position)
            .take(len)
            .collect::<String>();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        let mut len = 0;
        while self.ch.is_some() && self.is_digit(self.ch.unwrap()) {
            len += 1;
            self.read_char();
        }

        return self
            .input
            .chars()
            .skip(position)
            .take(len)
            .collect::<String>();
    }

    fn is_letter(&mut self, ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_digit(&mut self, ch: char) -> bool {
        ch.is_numeric()
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return None;
        }

        self.input.chars().nth(self.read_position)
    }

    fn is_known_single(&mut self, ch: char) -> bool {
        let known_chars: Vec<char> = vec![
            '=', ';', '(', ')', ',', '+', '{', '}', '-', '!', '/', '*', '<', '>',
        ];
        known_chars.contains(&ch)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.map(char::is_whitespace).unwrap_or_default() {
            self.read_char();
        }
    }

    fn process_char(&mut self, ch: char) -> Option<Token> {
        if self.is_known_single(ch) {
            let next_ch = self.peek_char();

            if (ch == '=' || ch == '!') && next_ch.is_some() {
                let next_ch = next_ch.unwrap();

                if next_ch == '=' {
                    self.read_char();
                    self.read_char();
                    let literal = format!("{}{}", ch, next_ch);
                    return Some(Token {
                        token_type: Token::lookup_token(literal.as_str()),
                        literal: String::from(literal),
                    });
                }
            }

            self.read_char();
            let literal = ch.to_string();

            Some(Token {
                token_type: Token::lookup_token(literal.as_str()),
                literal,
            })
        } else if self.is_letter(ch) {
            let literal = self.read_identifier();
            Some(Token {
                token_type: Token::lookup_token(literal.as_str()),
                literal,
            })
        } else if self.is_digit(ch) {
            Some(Token {
                token_type: TokenType::Integer,
                literal: self.read_number(),
            })
        } else {
            Some(Token {
                token_type: TokenType::Illegal,
                literal: String::from("illegal"),
            })
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let result: Option<Token> = match self.ch {
            Some(ch) => self.process_char(ch),
            _ => None,
        };

        return result;
    }
}
