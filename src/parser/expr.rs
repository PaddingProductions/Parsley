use crate::tokens::{Tokens, TokenType};
use crate::ast::*;

// expr : expr1
// expr1    : expr2 ((+|-) expr1)* 
//          ;
//
// expr2    : term ((*|/) expr2)*
//          ;
//
fn or<T, A> (tok: &mut Tokens, func: &dyn Fn( &mut Tokens, A ) -> Result<T, ()>, opts: Vec<A>) -> Result<T, ()> {
    for opt in opts {
        if let Ok(res) = func(tok, opt) {
            return Ok(res); 
        }
    }
    Err(())
}

fn operator (tok: &mut Tokens, s: String) -> Result<Operator, ()> {
    if tok[0].str() != s {
        Err(()) 
    } else {
        let op = match s.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multi,
            "/" => Operator::Div,
            _ => return Err(())
        };
        tok.pop_front();
        Ok(op)
    }
}

fn num (tok: &mut Tokens) -> Result<i64, ()> {
    if !matches!(tok[0].typ, TokenType::Num) {
        return Err(())
    }
    if let Ok(num) = tok[0].str().parse::<i64>() {
        tok.pop_front();
        return Ok(num);
    }
    return Err(());
}

impl Extractable for Expr {
    type T  = Self;
    fn extract (tok: &mut Tokens) -> Result<Self::T, ()> {
        let t0 = Self::expr2(tok)?;
        let mut v: Vec<(Operator, Box<dyn Evaluable>)> = vec![(Operator::Plus, Box::new(t0))];
        while let Ok(op) = or::<Operator, String> (tok, &operator, vec!["-".to_string(), "+".to_string()]) {
            let t = Self::expr2(tok)?;
            v.push((op, Box::new(t)));
        }
        Ok(Self::from(v))
    }
}

impl Expr {
    fn expr2 (tok: &mut Tokens) -> Result<Self, ()> {
        let t0 = num(tok)?;
        let mut v: Vec<(Operator, Box<dyn Evaluable>)> = vec![(Operator::Plus, Box::new(t0))];
        while let Ok(op) = or::<Operator, String> (tok, &operator, vec!["*".to_string(), "/".to_string()]) {
            let t = num(tok)?;
            v.push((op, Box::new(t)));
        }
        Ok(Self::from(v))
    }
}
