use super::BoxedParser;
use super::core::*;
use crate::ast::{Operation, Evaluable};
use crate::interpreter::{Environment, InterpreterErr};
use super::expr;


struct Print {
    e: expr::Expr 
}

impl Operation for Print {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        println!("{:?}", self.e.eval(env));
        Ok(())
    }
}

pub fn print<'a> () -> BoxedParser<'a, Box<dyn Operation>> {
    BoxedParser::new(  prefix("print ", expr::Expr::any)  )
        .map( |e| -> Box<dyn Operation> {Box::new(Print{ e })} )
}
