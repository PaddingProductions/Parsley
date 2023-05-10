use super::*;
use super::core::*;
use crate::ast::{Identifier, Evaluable, Operation};
use crate::interpreter::Environment;


struct Expr {
    t0: Box<dyn Evaluable<f64>>,
    v: Vec<(String, Box<dyn Evaluable<f64>>)>
}
impl Evaluable<f64> for Expr {
    fn eval (&self, env: &mut Environment) -> f64 {
        let mut res = self.t0.eval(env);
        for (op, t) in self.v.iter() {
            match op.as_str() {
                "+" => res += t.eval(env),
                "*" => res *= t.eval(env),
                "-" => res -= t.eval(env),
                "/" => res /= t.eval(env),
                _ => panic!("invalid operator character found in expression")
            }
        }
        res
    }
}

impl Evaluable<f64> for f64 {
    fn eval (&self, _env: &mut Environment) -> f64 { *self }
}
impl Evaluable<f64> for Identifier {
    fn eval (&self, env: &mut Environment) -> f64 {
        let var = env.vars.get(self).expect(&format!("'{}' not defined", self));
        var.clone()
    }
}

struct ExprOp { e: Box<dyn Evaluable<f64>> }
impl Operation for ExprOp {
    fn exec (&self, env: &mut Environment) {
        println!("Expression Evaluated to => '{}'", self.e.eval(env))
    }
}
pub fn expression_op<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    map(
        prefix(
            "eval:",
            expression(),   
        ),
        |e| -> Box<dyn Operation> { Box::new(ExprOp { e }) }
    )
}

pub fn expression<'a> () -> impl Parser<'a, Box<dyn Evaluable<f64>>> {
    map(
        and(
            precedence1(),
            zero_or_more(
                and(
                    parse_literals(vec!["+", "-"]),
                    precedence1()
                )
            )
        ),
        |(t0, v)| -> Box<dyn Evaluable<f64>> { 
            Box::new( Expr { 
                t0, 
                v: v.into_iter().map(|(op, v)| (String::from(op), v)).collect() 
            }
        )}
    )
}

fn precedence1<'a> () -> impl Parser<'a, Box<dyn Evaluable<f64>>> {
    map(
        and(
            term(),
            zero_or_more(
                and(
                    parse_literals(vec!["*", "/"]),
                    term()
                )
            )
        ),
        |(t0, v)| -> Box<dyn Evaluable<f64>> { 
            Box::new( Expr { 
                t0, 
                v: v.into_iter().map(|(op, v)| (String::from(op), v)).collect() 
            }
        )}
    )
}


fn term<'a> () -> impl Parser<'a, Box<dyn Evaluable<f64>>> {
    |buf| { 
        if let Ok((buf, num)) = parse_number().parse(buf) {
            Ok((buf, box_evaluable(num)))
        } else
        if let Ok((buf, ident)) = parse_identifier().parse(buf) {
            Ok ((buf, box_evaluable(ident)))
        } else 
        {
            par_err("Did not match 'term' grammar")
        }
    }
}

fn box_evaluable<T, E> (o: E) -> Box<dyn Evaluable<T>>
where
    E: Evaluable<T> + 'static
{
    let b: Box<dyn Evaluable<T>> = Box::new(o);
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_expression () {
        let mut env = Environment::new();
        let input1 = "1+2";
        let input2 = "1+2*3";
        let input3 = "1*2+3*4+4";

        assert!(expression().parse(input1).unwrap().1.eval(&mut env) == 3.0);
        assert!(expression().parse(input2).unwrap().1.eval(&mut env) == 7.0);
        assert!(expression().parse(input3).unwrap().1.eval(&mut env) == 18.0);
    }
}
