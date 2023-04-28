use std::boxed::Box;
use std::collections::HashMap;

use crate::ast::*;

pub trait Value {}

pub struct Environment {
    vars: HashMap<String, i64>
}

impl Environment {
    pub fn new () -> Self { 
        Self {
            vars: HashMap::new()
        } 
    }
}

impl Operation for Expr {
    fn exec (&self, env: &mut Environment) -> bool {
        println!("Expr Evaluated to: {}", self.eval(env)); 
        true
    }
}

impl Evaluable for Term {
    fn eval (&self, env: &mut Environment) -> i64 { 
        use Term::*;
        match self {
            Num(num)        => *num,
            Ident(ident)    => *env.vars.get(ident).expect("Use of undefined identifier.")
        }
    }
}

impl Evaluable for Expr {
    fn eval (&self, env: &mut Environment) -> i64 {
        let mut res: i64 = 0;
        for term in self.terms() {
            //println!("evaluating term: op: {:?}, recursing", term.0);
            let val = term.1.eval(env);
            match term.0 {
                Operator::Plus  => res += val, 
                Operator::Minus => res -= val, 
                Operator::Multi => res *= val, 
                Operator::Div   => res /= val, 
                _ => panic!("Unexpected non-arithmetic operator")
            }
        }
        res
    }   
}


impl Operation for Assignment {
    fn exec (&self, env: &mut Environment) -> bool {
        let val  = self.expr.eval(env);
        let prev = env.vars.insert(self.ident.clone(), val);
        println!("Wrote from {:?} -> {} to identifier {}", prev, val, self.ident);
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
