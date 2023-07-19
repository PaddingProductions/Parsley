use super::*;
use super::core::*;
use crate::ast::{Operation, Expr, Evaluable};
use crate::interpreter::{Environment, InterpreterErr};

use super::expr::expression;

struct Print {
    e: Expr
}

impl Operation for Print {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        println!("{:?}", self.e.eval(env));
        Ok(())
    }
}

pub fn print<'a> () -> BoxedParser<'a, Box<dyn Operation>> {
    BoxedParser::new( 
        prefix( "print ",
            expression()
        ) 
    )
        .map( |e| -> Box<dyn Operation> {Box::new(Print{ e })} )
}
