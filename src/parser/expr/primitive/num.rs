use std::sync::OnceLock;

use crate::parser::core::*;
use crate::parser::*;
use crate::interpreter::{ 
    inter_err,
    Environment, InterpRes
};
use crate::ast::Types;


const OPERATORS: [&str; 2] = [
    "+ -",
    "* / %",
];


pub trait Evaluable {
    fn eval (&self, env: &mut Environment) -> InterpRes<f64>;
}


enum Term {
    Num(f64),
    Ident(String),
}


impl Evaluable for Term {
    fn eval (&self, env: &mut Environment) -> InterpRes<f64> {
        match self {
            Num(v)   => Ok( v.clone() ),
            Ident(b) => env.var(b)?.clone().to_num(),
        }
    }
}
use Term::*;


pub struct Expr {
    t0: Box<dyn Evaluable>,
    v: Vec<(String, Expr)>
}


impl Types {
    fn to_num (self) -> InterpRes<f64> {
        match self {
            Types::Num(v) => Ok(v),
            _ => inter_err("Term {self} does not resolve to a number")
        }
    }
}


impl Evaluable for Expr {
    fn eval (&self, env: &mut Environment) -> InterpRes<f64> {
        // Evaluate first term
        let mut res = self.t0.eval(env)?;

        for (op, t) in self.v.iter() {
            let v = t.eval(env)?;

            match op.as_str() {
                "+" => res += v,
                "-" => res -= v,
                "*" => res *= v,
                "/" => res /= v,
                "%" => res %= v,

                /* This should be filtered during the parsing phase */
                _ => return inter_err("{op} is not a valid NUM operator")             
            }
        }
        Ok(res)
    }
}

// == PARSERS ==
static cell: OnceLock<BoxedParser<Expr>> = OnceLock::new();
pub fn expr<'a> (buf: &'a str) -> ParseRes<'a, Expr> {
    cell.get_or_init( || operand(0) );

    cell.get().unwrap().parse(buf)
}

fn operator<'a> (p: usize) -> BoxedParser<'a, String> {
    BoxedParser::new(
        parse_literals(OPERATORS[p].split(' ').collect()))
            .map(|s| String::from(s)
    )
}

fn operand<'a> (p: usize) -> BoxedParser<'a, Expr> {
    if p == OPERATORS.len() {
        return 
            BoxedParser::new(term)
                .map(|evaluable| Expr { t0: evaluable, v: vec![] } )
    }
    operand(p+1)
        .and(
        operator(p) 
            .and(operand(p+1))
            .zero_or_more()
        )
        .map(
            |(t0, v)| Expr { 
                t0: Box::new(t0), 
                v
            })
}


fn term<'a> (buf: &'a str) -> ParseRes<'a, Box<dyn Evaluable>> {
    let mut iter = buf.chars().peekable();
    
    // If negation 
    // let negation = if let Ok((b, _)) = parse_literal("-").parse(buf) { buf = b; true } else { false };

    // If parenthesis, parse expression
    if iter.peek() == Some(&'(') {
        return expr.parse(&buf[1..])
                   .map(|(buf, op)| -> (_, Box<dyn Evaluable>) {
                        (&buf[1..], Box::new(op))
                    })
    }

    // If Identifier 
    if let Ok((buf, o)) = parse_identifier(buf) {
        return Ok((buf, Box::new(Ident(o))));
    }

    // If Num Literal 
    if let Ok((buf, n)) = parse_number(buf) {
        return Ok((buf, Box::new(Num(n))));
    }   

    par_err(buf, "no base found")
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_num_expr () {
        let mut env = Environment::new();
        let input1 = "1+2";
        let input2 = "1+2*3";
        let input3 = "1*2+3*4+4";
        let input4 = "1*(2+3)*(4+4)";

        let expr = BoxedParser::new(expr);
        
        assert!(expr.test(input1).eval(&mut env).unwrap() == 3.0);
        assert!(expr.test(input2).eval(&mut env).unwrap() == 7.0);
        assert!(expr.test(input3).eval(&mut env).unwrap() == 18.0);
        assert!(expr.test(input4).eval(&mut env).unwrap() == 40.0);
    }
}
