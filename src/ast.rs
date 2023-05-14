use crate::interpreter::Environment;
use std::boxed::Box;

pub trait Operation {
    fn exec (&self, env: &mut Environment) -> Result<(),()>;
}

pub struct Block {
    pub ops: Vec<Box<dyn Operation>>
}

pub struct Assignment {
    pub ident: String, 
    pub expr: Box<dyn Evaluable>
}

pub type Identifier = String;

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Num (f64),
    Bool (bool),
    Ident (String)
}

pub trait Evaluable {
    fn eval (&self, env: &mut Environment) -> Result<Types, ()>;
}
impl Evaluable for Types {
    fn eval (&self, _env: &mut Environment) -> Result<Types, ()> { Ok(self.clone()) } 
}
impl Operation for dyn Evaluable {
    fn exec (&self, env: &mut Environment) -> Result<(), ()> {
        println!("expression evaluated to: {:?}", self.eval(env));
        Ok(())
    }
}


pub struct If {
    pub expr: Box<dyn Evaluable>,
    pub block: Block, 
}


