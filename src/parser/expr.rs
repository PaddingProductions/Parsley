use crate::parser::*;
use crate::ast::*;
use crate::interpreter::*;


const OPERATORS: [&str; 5] = [
    "== !=",
    "||",
    "&&",
    "+ -",
    "* / %",
];

enum Term {
    Nil,
    Num(f64),
    Bool(bool),
    Ident(String),
}
impl Evaluable for Term {
    fn eval (&self, env: &mut Environment) -> Result<Types, InterpreterErr> {
        match self {
            Nil => Ok(Types::Nil),
            Num(v) => Ok(Types::Num(v.clone())),
            Bool(b) => Ok(Types::Bool(b.clone())),
            Ident(b) => {
                if let Some(t) = env.vars.get(b) {
                    Ok(t.clone())
                } else {
                    Err(inter_err("cannot resolve variable {self}"))
                }
            },
        }
    }
}
use Term::*;

pub struct Expr {
    t0: Box<dyn Evaluable>,
    v: Vec<(String, Expr)>
}

impl Types {
    fn to_num (&self) -> Result<f64, InterpreterErr> {
        match self {
            Types::Num(v) => Ok(v.clone()),
            _ => Err(inter_err("Term {self} does not resolve to a number"))
        }
    }

    fn to_bool (&self) -> Result<bool, InterpreterErr> {
        match self {
            Types::Bool(v) => Ok(v.clone()),
            _ => Err(inter_err("Term {self} does not resolve to a number"))
        }
    }
}

impl Evaluable for Expr {
    fn eval (&self, env: &mut Environment) -> Result<Types, InterpreterErr> {
        // Evaluate first term
        let t0 = self.t0.eval(env)?;

        // Determine expression type from first term
        match t0 {
            Types::Num(mut res) => {
                for (op, t) in self.v.iter() {
                    let v = t.eval(env)?.to_num()?;

                    match op.as_str() {
                        "==" => return Ok(Types::Bool(res == v)),
                        "!=" => return Ok(Types::Bool(res != v)),
                        "+" => res += v,
                        "-" => res -= v,
                        "*" => res *= v,
                        "/" => res /= v,
                        "%" => res %= v,
                        _ => return Err(inter_err("{op} is not a valid NUM operator")) //"Not a valid Num() operator"
                    }
                }
                Ok(Types::Num(res))
            },
            Types::Bool(mut res) => {
                for (op, t) in self.v.iter() {
                    let b = t.eval(env)?.to_bool()?;

                    match op.as_str() {
                        "==" => return Ok(Types::Bool(res == b)),
                        "!=" => return Ok(Types::Bool(res != b)),
                        "&&" => res &= b,
                        "||" => res |= b,
                        _ => return Err(inter_err("{op} is not a valid BOOL operator")) //"Not a valid Bool() operator"
                    }
                }
                Ok(Types::Bool(res))
            }
            
            /* We allow a "a = nil" assignment, but disallow operations on nil. */
            Types::Nil => {
                if self.v.is_empty() {
                    Ok(Types::Nil)
                } else {
                    Err(inter_err("cannot operate on NIL"))
                }
            },
        }
    }
}


/// == PARSERS ==
pub fn expression<'a> () -> BoxedParser<'a, Expr> {
    operand(0)
}

fn operator<'a> (p: usize) -> BoxedParser<'a, String> {
    return BoxedParser::new(parse_literals(OPERATORS[p].split(' ').collect()))
        .map(|s| String::from(s))
}

fn operand<'a> (p: usize) -> BoxedParser<'a, Expr> {
    if p == OPERATORS.len() {
        return 
            BoxedParser::new(term())
                .map(|evaluable| Expr { t0: evaluable, v: vec![] } )
    }
    operand(p+1)
        .and(operator(p) 
                .and(operand(p+1))
                .zero_or_more()
        )
        .map(
            |(t0, v)| Expr { 
                t0: Box::new(t0), 
                v
            })
}


fn term<'a> () -> impl Parser<'a, Box<dyn Evaluable>> {
    |buf: &'a str| -> ParseRes<'a, Box<dyn Evaluable>> {
        let mut iter = buf.chars().peekable();
        
        // If negation 
        // let negation = if let Ok((b, _)) = parse_literal("-").parse(buf) { buf = b; true } else { false };

        // If parenthesis, parse expression
        if iter.peek() == Some(&'(') {
            return expression()
                .parse(&buf[1..])
                .map(|(buf, op)| -> (_, Box<dyn Evaluable>) {
                        (&buf[1..], Box::new(op))
                    })
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

        // If Bool Literal
        if let Ok((buf, s)) = parse_literals(vec!["true", "false"]).parse(buf) {
            return Ok((buf, Box::new(Bool(s == "true"))));
        }   

        // If Nil Literal
        if let Ok((buf, _)) = parse_literal("nil").parse(buf) {
            return Ok((buf, Box::new(Nil)));
        }   

        par_err(buf, "no base found")
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
