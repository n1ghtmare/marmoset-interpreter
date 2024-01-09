use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement {
    fn statement_node(&self) -> String;
}

impl<'a> Node for dyn Statement + 'a {
    fn token_literal(&self) -> String {
        todo!()
    }
}

pub trait Expression {
    fn expression_node(&self) -> String;
}

impl<'a> Node for dyn Expression + 'a {
    fn token_literal(&self) -> String {
        todo!()
    }
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

struct Identifier {
    token: Token, // the token IDENT
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) -> String {
        todo!()
    }
}

pub struct LetStatement {
    token: Token, // the token LET
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) -> String {
        todo!()
    }
}
