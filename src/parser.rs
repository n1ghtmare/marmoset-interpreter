use crate::ast::*;
use crate::lexer::*;
use crate::token::*;

struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(&self, lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        return p;
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_program(&self) -> Program {
        let program = Program { statements: vec![] };

        while self.current_token.clone().unwrap().token_type != TokenType::Eof {
            let statement = self.parse_statement();
            if statement.is_some() {
                program.statements.push(statement);
            }
        }
        program
    }

    fn parse_statement(&self) -> Option<Box<dyn Statement>> {
        if !self.current_token.is_some() {
            return None;
        }

        let token = self.current_token.unwrap();
        match token.token_type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&self) -> Option<Box<LetStatement>> {
        todo!()
    }
}
