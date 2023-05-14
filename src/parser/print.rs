use super::*;
use super::core::*;
use crate::{ast::{Operation, Evaluable}, interpreter::Environment};

use super::expr::expression;

struct Print<T> 
where
    T: std::fmt::Display
{
    e: Box<dyn Evaluable<T>>
}

impl<T> Operation for Print<T> 
where 
    T: std::fmt::Display
{
    fn exec (&self, env: &mut Environment) {
        println!("{}", self.e.eval(env));
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
