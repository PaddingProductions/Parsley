use std::boxed::Box;

use crate::ast::*;

pub struct Environment {}
impl Environment {
    pub fn new () -> Self { Self {} }
}

impl Operation for Expr {
    fn exec (&self, env: &mut Environment) -> bool {
        println!("Expr Operation returned: {}", self.eval(env)); 
        true
    }
}

impl Evaluable for i64 {
    fn eval (&self, _: &mut Environment) -> Self { 
        println!("num literal, returning: {}", *self);
        *self
    }
}
impl Evaluable for Expr {
    fn eval (&self, env: &mut Environment) -> i64 {
        let mut res: i64 = 0;
        for term in self.terms() {
            println!("evaluating term: op: {:?}, recursing", term.0);
            let val = term.1.eval(env);
            match term.0 {
                Operator::Plus  => res += val, 
                Operator::Minus => res -= val, 
                Operator::Multi => res *= val, 
                Operator::Div   => res /= val, 
            }
        }
        res
    }   
}


impl Operation for Assignment {
    fn exec (&self, env: &mut Environment) -> bool {
        // TODO
        true
    }
}

pub struct Interpreter {
    pub env: Environment
}
impl Interpreter {
    pub fn new () -> Self {
        Self { env: Environment::new() }
    }

    pub fn interpret (&mut self, instructions: Vec<Box<dyn Operation>>) {
        for instruct in instructions.iter() {
            instruct.exec(&mut self.env); 
        }
    }
}
