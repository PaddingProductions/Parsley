use std::sync::OnceLock;

use crate::parser::core::*;
use crate::parser::*;
use crate::interpreter::{ 
    inter_err,
    Environment, InterpRes
};
use crate::ast::Types;


const OPERATORS: [&str; 2] = [
    "||",
    "&&",
];


pub trait Evaluable {
    fn eval (&self, env: &mut Environment) -> InterpRes<bool>;
}


enum Term {
    Bool(bool),
    Ident(String),
}


impl Evaluable for Term {
    fn eval (&self, env: &mut Environment) -> InterpRes<bool> {
        match self {
            Bool(v)   => Ok( v.clone() ),
            Ident(b) => env.var(b)?.clone().to_bool(),
        }
    }
}
use Term::*;


pub struct Expr {
    t0: Box<dyn Evaluable>,
    v: Vec<(String, Expr)>
}


impl Types {
    fn to_bool (self) -> InterpRes<bool> {
        match self {
            Types::Bool(v) => Ok(v),
            _ => inter_err("Term {self} does not resolve to a number")
        }
    }
}


impl Evaluable for Expr {
    fn eval (&self, env: &mut Environment) -> InterpRes<bool> {
        // Evaluate first term
        let mut res = self.t0.eval(env)?;

        for (op, t) in self.v.iter() {
            let v = t.eval(env)?;

            match op.as_str() {
                "||" => res |= v,
                "&&" => res &= v,

                /* This should be filtered during the parsing phase */
                _ => return inter_err("{op} is not a valid NUM operator")             
            }
        }
        Ok(res)
    }
}


/// == PARSERS ==
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
        return BoxedParser::new(map(term, |evaluable| Expr { t0: evaluable, v: vec![] } ))
    }
    operand(p+1).and(
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


    // If Bool Literal
    if let Ok((buf, s)) = parse_literals(vec!["true", "false"]).parse(buf) {
        return Ok((buf, Box::new(Bool(s == "true"))));
    }   


    par_err(buf, "no base found")
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_bool_expression () {
        let mut env = Environment::new();
        let input1 = "true && false";
        let input2 = "true || false";
        let input3 = "true || false && true";
        let input4 = "((false || true) || false) && true";

        let expr = BoxedParser::new(expr);

        assert!(expr.test(input1).eval(&mut env).unwrap() == false);
        assert!(expr.test(input2).eval(&mut env).unwrap() == true);
        assert!(expr.test(input3).eval(&mut env).unwrap() == true);
        assert!(expr.test(input4).eval(&mut env).unwrap() == true);
    }
}
