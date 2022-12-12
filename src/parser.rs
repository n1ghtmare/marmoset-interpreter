use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

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
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&self) -> Option<Program> {
        let program = Program {
            statements: Vec::new(),
        };

        todo!();
        // while self.current_token.clone().unwrap().token_type != TokenType::Eof {
        //     let statement = self.parse_statement();
        //     if statement.is_some() {
        //         // program.statements.push(statement);
        //     }
        // }
        Some(program)
    }

    fn parse_statement(&self) -> Option<Box<dyn Statement>> {
        if !self.current_token.is_some() {
            return None;
        }

        todo!();
        let token = self.current_token.unwrap();
        match token.token_type {
            TokenType::Let => todo!(),
            _ => None,
        }
    }

    fn parse_let_statement(&self) -> Option<Box<LetStatement>> {
        todo!()
    }
}
