use crate::interpreter::Environment;
use std::boxed::Box;

pub trait Operation {
    fn exec (&self, env: &mut Environment);
}

pub struct Block {
    pub ops: Vec<Box<dyn Operation>>
}

pub struct Assignment {
    pub ident: String, 
    pub expr: Box<dyn Evaluable<f64>>
}


pub type Identifier = String;
pub trait Evaluable<T> {
    fn eval (&self, env: &mut Environment) -> T;
}

impl<T> Operation for dyn Evaluable<T> 
where 
    T: std::fmt::Debug
{
    fn exec (&self, env: &mut Environment) {
        println!("expression evaluated to: {:?}", self.eval(env));
    }
}


pub struct If {
    pub expr: Box<dyn Evaluable<bool>>,
    pub block: Block, 
}


