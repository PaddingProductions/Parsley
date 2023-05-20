use crate::ast::{ Loop, Operation, Types };
use crate::interpreter::{InterpreterErr, Environment};

use super::BoxedParser;
use super::block::block;
use super::expr::expression;
use super::core::parse_literal;

impl Operation for Loop {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        while let Types::Bool(b) = self.expr.eval(env)? {
            if !b { break }
            self.block.exec(env)?;
        }
        Ok(())
    }
}

pub fn _while<'a> () -> BoxedParser<'a, Loop> {
    BoxedParser::new(parse_literal("while "))
        .and(expression())
        .and(block())
        .map(|((_, expr), block)| Loop{ expr, block })
}
