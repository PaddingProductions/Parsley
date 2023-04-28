use crate::tokens::TokIter;
use super::{ParserRes, ParserErr};
use crate::ast::*;
use super::core::*; 

// expr : expr1
// expr1    : expr2 ((+|-) expr1)* 
//          ;
//
// expr2    : term ((*|/) expr2)*
//          ;
//
fn term<'a, 'b> (tok: &'a TokIter<'b>) -> ParserRes<'b, Term> {
    if let Ok((tok, num)) = num(tok) {
        Ok((tok, Term::Num(num)))
    } 
    else if let Ok((tok, ident)) = ident(tok) {
        Ok((tok, Term::Ident(ident)))
    } else {
        Err(ParserErr::new("Unable to find valid 'term' grammar"))
    }
}

impl Extractable for Expr {
    type T = Self;
    fn extract<'a, 'b> (tok: &'a TokIter<'b>) -> ParserRes<'b, Expr> {
        println!("attempting to extract 'expr'");
        let tok: TokIter<'b> = tok.clone();
        let (mut tok, t0): (TokIter<'b>, Expr) = Self::expr2(&tok)?;
        let mut v: Vec<(Operator, Box<dyn Evaluable>)> = vec![(Operator::Plus, Box::new(t0))];
        loop {
            let op = match
                operator(&tok, "+") .or(
                operator(&tok, "-")) 
            {
                Ok((t, o)) => { tok = t; o},
                Err(_) => break
            };
            let t = match Self::expr2(&tok) {
                Ok((t, o)) => { tok = t; o},
                Err(e) => return Err(e)
            };
            v.push((op, Box::new(t)));
        }
        Ok((tok, Self::from(v)))
    }
}

impl Expr {
    fn expr2<'a, 'b> (tok: &'a TokIter<'b>) -> ParserRes<'b, Expr> {
        println!("attempting to extract 'expr2'");
        let tok = tok.clone();
        let (mut tok, t0) = term(&tok)?;
        let mut v: Vec<(Operator, Box<dyn Evaluable>)> = vec![(Operator::Plus, Box::new(t0))];
        loop {
            let op = match  
                operator(&tok, "*") .or( 
                operator(&tok, "/")).or(
                operator(&tok, "%")) 
            {
                Ok((t, o)) => {tok = t; o},
                Err(_) => break
            };
             let t = match term(&tok) {
                Ok((t, o)) => {tok = t; o},
                Err(e) => return Err(e)
            };
            v.push((op, Box::new(t)));
        }
        Ok((tok, Self::from(v)))    
    }
}
