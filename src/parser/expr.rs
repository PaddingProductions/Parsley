use super::*;
use super::core::*;
use crate::ast::*;
use crate::interpreter::*;

use Types::*;

const OPERATORS: [&str; 5] = [
    "== !=",
    "||",
    "&&",
    "+ -",
    "* / %",
];

struct Expr {
    t0: Box<dyn Evaluable>,
    v: Vec<(String, Box<dyn Evaluable>)>
}

fn resolve_into_num (env: &mut Environment, v: Types) -> Result<f64, InterpreterErr>{
    match v {
        Num(v) => Ok(v),
        Ident(s) => {
            let v = env.vars.get(&s).ok_or(inter_err(format!("Variable {} undefined", s).as_str()))?;
            if let Num(x) = v { Ok(*x) } else { return Err(inter_err("Variable not a number")) } 
        },
        _ => Err(inter_err("Cannot resolve term to type Num()"))
    }
}
fn resolve_into_bool (env: &mut Environment, v: Types) -> Result<bool, InterpreterErr>{
    match v {
        Bool(b) => Ok(b),
        Ident(s) => {
            let v = env.vars.get(&s).ok_or(inter_err(format!("Variable {} undefined", s).as_str()))?;
            if let Bool(b) = v { Ok(*b) } else { return Err(inter_err("Variable not a boolean")) }
        },
        _ => Err(inter_err("Cannot resolve term to type Bool()"))
    }
}

impl Evaluable for Expr {
    fn eval (&self, env: &mut Environment) -> Result<Types, InterpreterErr> {
        let t0 = self.t0.eval(env)?;

        // Resolve Identifier
        let t0 = if let Ident(s) = t0 { 
            env.vars.get(&s).expect(&format!("Attempted to resolve undefined variable {}", s)).clone()
        } else { t0 };

        // Switch between type expressed.
        match t0 {
            Num(mut res) => {
                for (op, t) in self.v.iter() {
                    let v = t.eval(env)?;
                    let v = resolve_into_num(env, v)?;

                    match op.as_str() {
                        "==" => return Ok(Bool(res == v)),
                        "!=" => return Ok(Bool(res != v)),
                        "+" => res += v,
                        "-" => res -= v,
                        "*" => res *= v,
                        "/" => res /= v,
                        "%" => res %= v,
                        _ => return Err(inter_err("Not a Num() operator")) //"Not a valid Num() operator"
                    }
                }
                Ok(Num(res))
            },
            Bool(mut res) => {
                for (op, t) in self.v.iter() {
                    let b = t.eval(env)?;
                    let b = resolve_into_bool(env, b)?;

                    match op.as_str() {
                        "==" => return Ok(Bool(res == b)),
                        "!=" => return Ok(Bool(res != b)),
                        "&&" => res &= b,
                        "||" => res |= b,
                        _ => return Err(inter_err("Not a Bool() operator")) //"Not a valid Bool() operator"
                    }
                }
                Ok(Bool(res))
            }
            Ident(_) => panic!("t0 still an Ident(). This should never happen."),
            Nil => return Ok(Nil),
            _ => panic!("not implemented yet!")
        }
    }
}


pub fn expression<'a> () -> BoxedParser<'a, Box<dyn Evaluable>> {
    term(0)
}

fn term<'a> (p: usize) -> BoxedParser<'a, Box<dyn Evaluable>> {
    if p == OPERATORS.len() {
        return BoxedParser::new(base());
    }
    term(p+1)
        .and( 
            BoxedParser::new(parse_literals(OPERATORS[p].split(' ').collect()))
                .and(term(p+1))
                .zero_or_more()
        )
        .map(
            |(t0, v)| -> Box<dyn Evaluable> { 
                Box::new( Expr { 
                    t0, 
                    v: v.into_iter().map(|(op, v)| (String::from(op), v)).collect() 
                }
            )}
        )
}


fn base<'a> () -> impl Parser<'a, Box<dyn Evaluable>> {
    |buf: &'a str| -> ParseRes<'a, Box<dyn Evaluable>> {
        let mut iter = buf.chars().peekable();
        let mut counter = 0;
        while iter.peek() == Some(&' ') { 
            counter += 1;
            iter.next();
        }
        let buf = &buf[counter..];

        // If parenthesis
        if iter.peek() == Some(&'(') {
            return expression().parse(&buf[1..])
                .map(|(buf, op)| (&buf[1..], op));
        }

        // If conditional
        if let Ok(_) = parse_literal("if ").parse(buf) {
            return conditional::conditional_if()
                .map(|o| -> Box<dyn Evaluable> { Box::new(o) })
                .parse(buf)
        }   

        // If block
        if iter.peek() == Some(&'{') {
            return block::block()
                .map(|o| -> Box<dyn Evaluable> { Box::new(o) })
                .parse(buf)
        }   

        // If Identifier 
        if let Ok((buf, o)) = parse_identifier().parse(buf) {
            return Ok((buf, Box::new(Ident(o))));
        }

        // If Num Literal 
        if let Ok((buf, n)) = parse_number().parse(buf) {
            return Ok((buf, Box::new(Num(n))));
        }   

        if let Ok((buf, s)) = parse_literals(vec!["true", "false"]).parse(buf) {
            return Ok((buf, Box::new(Bool(s == "true"))));
        }   

        par_err("no base found")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Types;
    use crate::interpreter::Environment;

    #[test] 
    fn test_expression () {
        let mut env = Environment::new();
        let input1 = "1+2";
        let input2 = "1+2*3";
        let input3 = "1*2+3*4+4";

        assert!(expression().parse(input1).unwrap().1.eval(&mut env).unwrap() == Types::Num(3.0));
        assert!(expression().parse(input2).unwrap().1.eval(&mut env).unwrap() == Types::Num(7.0));
        assert!(expression().parse(input3).unwrap().1.eval(&mut env).unwrap() == Types::Num(18.0));
    }

    #[test] 
    fn test_bool_expression () {
        let mut env = Environment::new();
        let input1 = "1==2";
        let input2 = "2==2";
        let input3 = "2!=2";
        let input4 = "1*2+3==4+4-3";

        assert!(expression().parse(input1).unwrap().1.eval(&mut env).unwrap()== Types::Bool(false));
        assert!(expression().parse(input2).unwrap().1.eval(&mut env).unwrap()== Types::Bool(true));
        assert!(expression().parse(input3).unwrap().1.eval(&mut env).unwrap()== Types::Bool(false));
        assert!(expression().parse(input4).unwrap().1.eval(&mut env).unwrap()== Types::Bool(true));
    }
}
