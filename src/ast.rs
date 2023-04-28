use crate::interpreter::Environment;
use crate::tokens::TokIter;
use crate::parser::ParserRes;

pub trait Operation {
    fn exec (&self, env: &mut Environment) -> bool;
}

pub trait Evaluable {
    fn eval (&self, env: &mut Environment) -> i64;
}

pub trait Extractable {
    type T;
    fn extract<'a, 'b> (tok: &'a TokIter<'b>) -> ParserRes<'b, Self::T>;
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multi,
    Div,
    Equal
}

#[derive(Debug)]
pub enum Term {
    Num (i64),
    Ident (String)
}

pub struct Expr {
    v: Vec<(Operator, Box<dyn Evaluable>)>
}


impl Expr {
    pub fn from (v: Vec<(Operator, Box<dyn Evaluable>)>) -> Self {
        if v.is_empty() {
            panic!("Empty vector passed to Expr::from()");
        }
        if !matches!(v[0].0, Operator::Plus) {
            panic!("First term passed to Expr::from() does not have Operator::Plus");
        }
        Self { v }
    }
    pub fn terms (&self) -> &Vec<(Operator, Box<dyn Evaluable>)> { &self.v } 
}


pub struct Assignment {
    pub ident: String,
    pub expr: Box<dyn Evaluable>
}
