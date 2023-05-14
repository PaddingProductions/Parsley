use super::*;
use super::core::*;
use crate::ast::{Identifier, Evaluable};
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

pub fn expression<'a> () -> BoxedParser<'a, Box<dyn Evaluable<f64>>> {
    precedence1()
        .and(
            BoxedParser::new(parse_literals(vec!["+", "-"]))
                .and(precedence1())
                .zero_or_more()
        )
        .map(
            |(t0, v)| -> Box<dyn Evaluable<f64>> { 
                Box::new( Expr { 
                    t0, 
                    v: v.into_iter().map(|(op, v)| (String::from(op), v)).collect() 
                })
            }
        )
}

fn precedence1<'a> () -> BoxedParser<'a, Box<dyn Evaluable<f64>>> {
    term()
        .and( 
            BoxedParser::new(parse_literals(vec!["*", "/"]))
                .and(term())
                .zero_or_more()
        )
        .map(
            |(t0, v)| -> Box<dyn Evaluable<f64>> { 
                Box::new( Expr { 
                    t0, 
                    v: v.into_iter().map(|(op, v)| (String::from(op), v)).collect() 
                }
            )}
        )
}


fn term<'a> () -> BoxedParser<'a, Box<dyn Evaluable<f64>>> {
    BoxedParser::new(or( 
        map(parse_number(), |num| box_evaluable(num)),
        map(parse_identifier(), |ident| box_evaluable(ident))
    ))
}

pub fn box_evaluable<T, E> (o: E) -> Box<dyn Evaluable<T>>
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
