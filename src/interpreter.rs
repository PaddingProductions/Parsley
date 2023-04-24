use std::boxed::Box;

use crate::ast::*;

pub struct Interpreter {}

impl Interpreter {
    pub fn new () -> Interpreter {
        Interpreter { }
    }

    pub fn interpret (&mut self, instructions: Vec<Box<dyn Instruction>>) {
        for instruct in instructions.iter() {
            let expr: &Expression = instruct.as_any().downcast_ref::<Expression>().expect("Instruction Not Expression");
            println!("Eval result: {}\n", self.exec(expr));
        }
    }

    fn exec (&mut self, expr: &Expression) -> usize {
        let mut res = 0;
        for operation in expr.terms.iter() {
            let term =
                if let Evaluable::Literal( literal ) = operation.term {
                    literal
                    //println!("Op on Literal: {:?} {:?}", operation.op, literal);
                }  
                else if let Evaluable::Expr( expr ) = &operation.term {
                    //println!("Op on Expression: {:?}, recursing", operation.op);
                    self.exec(&expr)
                }
                else {
                    panic!("Term with no evaluation method defined");
                };
            match operation.op.as_str() {
                "+" => res += term,
                "-" => res -= term,
                "*" => res *= term,
                "/" => res /= term,
                _ => panic!("Unknown operator")
            }
        }
        res 
    }
}
