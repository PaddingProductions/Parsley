use crate::interpreter::{Environment, InterpreterErr};
use std::boxed::Box;

pub trait Operation {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr>;
}

pub struct Block {
    pub ops: Vec<Box<dyn Operation>>,
    pub ret_expr: Option<Box<dyn Evaluable>>
} 
impl Block {
    pub fn empty () -> Self {
        Block { ops: vec![], ret_expr: None }
    }
} 

pub struct Assignment {
    pub ident: String, 
    pub expr: Box<dyn Evaluable>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Nil,
    Num (f64),
    Bool (bool),
    Ident (String)
}

pub trait Evaluable {
    fn eval (&self, env: &mut Environment) -> Result<Types, InterpreterErr>;
}
impl Evaluable for Types {
    fn eval (&self, _env: &mut Environment) -> Result<Types, InterpreterErr> { Ok(self.clone()) } 
}
impl<T: Evaluable> Operation for T {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        println!("expression evaluated to: {:?}", self.eval(env)?);
        Ok(())
    }
}


pub struct If {
    pub expr: Box<dyn Evaluable>,
    pub block: Block, 
    pub else_block: Block,
}

pub struct Loop {
    pub expr: Box<dyn Evaluable>,
    pub block: Block,
}

