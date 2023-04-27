use std::boxed::Box;
use std::collections::VecDeque;

use crate::tokens::*;
use crate::ast::*;

mod expr;
mod assign;

// Grammars
// prog : (op)* EOF 
// term : BOOL
//      | NUM
//      | IDENT
//
// op   : expr
//      | assign
//
// expr : <expr.rs>
// assign   : <assign.rs>
//

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn from (tokens: VecDeque<Token>) -> Parser {
        Parser{ tokens }
    }

    pub fn parse (mut self) -> Vec<Box<dyn Operation>> { 
        let mut list: Vec<Box<dyn Operation>> = vec![];
        while !matches!(self.tokens[0].typ, TokenType::EOF) {
            list.push(self.operation());
        }
        list
    }

    fn operation (&mut self) -> Box<dyn Operation> {
        let tok = &mut self.tokens;
        if let Ok(op) = Assignment::from(tok) {
            Box::new( op )
        } else {
            Box::new( Expr::extract(tok).expect("No valid grammars found in parsing 'operation()'") )
        }
    }
}
