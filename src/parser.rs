use std::boxed::Box;

use crate::tokens::*;
use crate::ast::*;

mod core;
mod expr;
mod assign;

#[derive(Debug)]
pub struct ParserErr {
    msg: String
}
impl ParserErr {
    pub fn new (s: &str) -> Self {
        Self { msg: s.to_string() }
    }
}
impl std::fmt::Display for ParserErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parser Err: {}", self.msg)
    }
}
impl std::error::Error for ParserErr {}

pub type ParserRes<'a, T> = Result<(TokIter<'a>, T), ParserErr>;

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
    tokens: Tokens,
}

impl Parser {
    pub fn from (tokens: Tokens) -> Parser {
        Parser{ tokens }
    }

    pub fn parse (self) -> Vec<Box<dyn Operation>> { 
        let mut list: Vec<Box<dyn Operation>> = vec![];
        let mut iter = self.tokens.iter().peekable();
        while iter.peek().is_some() && !matches!(iter.peek().unwrap().typ, TokenType::EOF) {
            if let Ok((t, o)) = Self::operation(&iter) {
                iter = t;
                list.push(o);
            } else { break; }
        }
        list
    }

    fn operation<'a, 'b> (tok: &'a TokIter<'b>) -> ParserRes<'b, Box<dyn Operation>> {
        if let Ok((tok, o)) = Assignment::extract(tok) {
            Ok((tok, Box::new(o)))
        } else 
        if let Ok((tok, o)) = Expr::extract(tok) {
            Ok((tok, Box::new(o)))
        }
        else {
            panic!("No valid operation found");
        }
    }
}
