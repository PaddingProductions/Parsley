use std::any::Any;

pub trait Instruction {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub enum Evaluable {
    Literal (usize),
    Expr (Expression)
}

#[derive(Debug)]
pub struct Expression {
    pub terms: Vec<Operation>
}
impl Instruction for Expression {
    fn as_any (&self) -> &dyn Any { self }
}

#[derive(Debug)]
pub struct Operation {
    pub op: String,
    pub term: Evaluable 
}

