use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while let Some(token) = &self.current_token {
            if token.token_type == TokenType::Eof {
                break;
            }

            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }

            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&self) -> Option<Box<dyn Statement>> {
        if let Some(token) = &self.current_token {
            match token.token_type {
                TokenType::Let => self
                    .parse_let_statement()
                    .map(|statement| Box::new(statement) as Box<dyn Statement>),
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&self) -> Option<LetStatement> {
        if let Some(token) = &self.current_token {
            let statement = LetStatement {
                token: token.clone(),
                name: Identifier {
                    token: token.clone(),
                    value: token.clone().literal,
                },
            };
            Some(statement)
        } else {
            None
        }
    }
}
