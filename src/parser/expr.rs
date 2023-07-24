use std::cell::OnceCell;
use std::sync::OnceLock;

use crate::interpreter::{
    Environment,
    InterpRes,
};
use crate::parser::*;
use crate::ast::Types;


pub trait Evaluable {
    fn eval (&self, env: &mut Environment) -> InterpRes<Types>;
}

pub enum Expr {
    Num (primitive::num::Expr),
    Bool(primitive::bol::Expr),
}
impl Expr {
    pub fn num<'a> (buf: &'a str) -> ParseRes<'a, Expr> {
        static cell: OnceLock<BoxedParser<Expr>> = OnceLock::new();
        cell.get_or_init( ||     
            BoxedParser::new(map( primitive::num::expr, |e| Expr::Num(e)))
        ).parse(buf)
    }

    pub fn bool<'a> (buf: &'a str) -> ParseRes<'a, Expr> {
        static cell: OnceLock<BoxedParser<Expr>> = OnceLock::new();
        cell.get_or_init( ||     
            BoxedParser::new(map( primitive::bol::expr, |e| Expr::Bool(e)))
        ).parse(buf)
    }

    pub fn any<'a> (buf: &'a str) -> ParseRes<'a, Expr> {
        static cell: OnceLock<BoxedParser<Expr>> = OnceLock::new();
        cell.get_or_init( ||     
            BoxedParser::new(or( Self::num, Self::bool ))
        ).parse(buf)
    }
}

impl Evaluable for Expr {
    fn eval (&self, env: &mut Environment) -> InterpRes<Types> {
        Ok(match self {
            Expr::Num(e)  => Types::Num (primitive::num::Evaluable::eval(e, env)?),
            Expr::Bool(e) => Types::Bool(primitive::bol::Evaluable::eval(e, env)?),
        })
    }
}

pub mod object;
pub mod primitive;
pub mod equality;
