use super::*;
use super::core::*;
use crate::{ast::{Operation, Evaluable}, interpreter::Environment};

use super::expr::expression;

struct Print {
    e: Box<dyn Evaluable>
}

impl Operation for Print {
    fn exec (&self, env: &mut Environment) -> Result<(), ()> {
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
