use std::boxed::Box;

use crate::tokens::*;
use crate::ast::*;


pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn from (tokens: Vec<Token>) -> Parser {
        Parser{ tokens }
    }

    pub fn parse (mut self) -> Vec<Box<dyn Instruction>> { 
        let mut list: Vec<Box<dyn Instruction>> = vec![];
        list.push(Box::new( self.expr(&mut 0, 1) )); 
        list
    }

    fn op_level (op: &String) -> usize {
        match op.as_str() {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => panic!("unknown operator")
        }
    }
    
    fn get_lit (&self, index: &mut usize) -> Evaluable {
        let lit = self.tokens[*index].str().parse::<usize>().expect("Expected numeric literal, parsing failed");
        *index += 1;
        Evaluable::Literal( lit )
    }

    fn get_op (&self, index: &mut usize) -> String {
        if !matches!(self.tokens[*index].typ, TokenType::Operator) {
            panic!("Expected Operator");
        } 
        let op = self.tokens[*index].str().to_string();
        *index += 1;
        op
    }

    fn endline (&self, index: &mut usize) -> bool {
        matches!(self.tokens[*index].typ, TokenType::End)
    }
    
    fn expr (&mut self, index: &mut usize, lv: usize) -> Expression { 
        let mut term = self.get_lit(index);
        if self.endline(index) {
            return Expression {
                terms: vec![ Operation {
                    op: "+".to_string(),
                    term
                }]
            }
        }
    
        {
            let op = self.get_op(index);
            if Self::op_level(&op) > lv {
                *index -= 2;
                term = Evaluable::Expr( self.expr(index, lv +1) );
            } else {
                *index -= 1;
            }
        }
        let mut terms = vec![ Operation { op: String::from("+"), term } ];

        while !self.endline(index) { 
            let op = self.get_op(index);

            if Self::op_level(&op) < lv {
                *index -= 1;
                break; 
            } 
            else if Self::op_level(&op) == lv {
                terms.push( Operation { 
                    op, 
                    term: self.get_lit(index) 
                });
            }
            else if Self::op_level(&op) > lv {
                *index -= 2;
                let new  = Evaluable::Expr( self.expr(index, lv+1) );
                let i = terms.len()-1;
                terms[i].term = new;
            }
        }

        println!("expr() result: {:?}", terms);
        Expression { terms }
    }
}
