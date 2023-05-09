use crate::interpreter::Environment;

pub trait Operation {
    fn exec (&self, env: &mut Environment);
}

pub struct PrintOperation {
    s: String
}
impl PrintOperation {
    pub fn new (s: &str) -> Self {
        Self { s: String::from(s) }
    }
}
impl Operation for PrintOperation {
    fn exec(&self, env: &mut Environment) {
        println!("{}", self.s);
    }
}

pub struct Block {
    pub ops: Vec<Box<dyn Operation>>
}

#[derive(PartialEq, Debug)]
pub struct Expression {
    pub t0: f64,
    pub v: Vec<(char, f64)>
}

#[derive(PartialEq, Debug)]
pub struct BoolExpression {
    pub a: Expression,
    pub b: Expression,
    pub op: String,
}

#[derive(PartialEq, Debug)]
pub struct Assignment {
    pub ident: String, 
    pub expr: Expression,
}

pub struct If {
    pub bexpr: BoolExpression,
    pub block: Block, 
}
