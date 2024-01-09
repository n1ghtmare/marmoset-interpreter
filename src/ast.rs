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
        if let Some(node) = self.statements.first() {
            node.token_literal()
        } else {
            String::from("")
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token, // the token IDENT
    pub value: String,
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

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token, // the token LET
    pub name: Identifier,
    // value: Box<dyn Expression>,
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
