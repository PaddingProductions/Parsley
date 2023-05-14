/*
use super::*;
use super::core::*;
use super::expr::expression;
use crate::ast::{Identifier, Evaluable};
use crate::interpreter::Environment;
use super::expr::box_evaluable;

struct EqExpr<T>
where 
    T: PartialEq 
{
    op: String,
    a: Box<dyn Evaluable<T>>,
    b: Box<dyn Evaluable<T>> 
}
impl<T> Evaluable<bool> for EqExpr<T>
where
    T: PartialEq
{
    fn eval (&self, env: &mut Environment) -> bool {
        let eq = self.a.eval(env) == self.b.eval(env);
        match self.op.as_str() {
            "==" => eq,
            "!=" => !eq,
            _ => panic!("invalid operator character found in equate expression")
        }
    }
}
impl Evaluable<bool> for bool {
    fn eval (&self, env: &mut Environment) -> bool { *self }
}

struct BoolExpr {
    t0: Box<dyn Evaluable<bool>>,
    v: Vec<(String, Box<dyn Evaluable<bool>>)>
}
impl Evaluable<bool> for BoolExpr {
    fn eval (&self, env: &mut Environment) -> bool {
        let mut res = self.t0.eval(env);
        for (op, t) in self.v.iter() {
            match op.as_str() {
                "&&" => res &= t.eval(env),
                "||" => res |= t.eval(env),
                _ => panic!("invalid operator character found in boolean expression")
            }
        }
        res
    }
}
impl Evaluable<bool> for Identifier {
    fn eval (&self, env: &mut Environment) -> bool {
        let var = env.vars.get(self).expect(&format!("'{}' not defined", self));
        *var != 0.0
    }
}

pub fn bool_expression<'a> () -> BoxedParser<'a, Box<dyn Evaluable<bool>>> {
    expression()
        .and(parse_literals(vec!["!=", "=="]))
        .and(expression())
        .map(
            |((a, op), b): ((Box<dyn Evaluable<f64>>, &str), Box<dyn Evaluable<f64>>)| -> Box<dyn Evaluable<bool>> {
                Box::new( EqExpr {op: String::from(op), a, b} )
            }
        )
    .or(
        precedence1()
            .and(parse_literals(vec!["!=", "=="]))
            .and(precedence1())
            .map(
                |((a, op), b): ((Box<dyn Evaluable<bool>>, &str), Box<dyn Evaluable<bool>>)| -> Box<dyn Evaluable<bool>> {
                    Box::new( EqExpr {op: String::from(op), a, b} )
                }
            )
        )
    .or(
        precedence1()
            .and(
                BoxedParser::new(parse_literal("&&"))
                    .and(precedence1())
                    .zero_or_more()
            ).map(
                |(t0, v): (Box<dyn Evaluable<bool>>, Vec<(&str, Box<dyn Evaluable<bool>>)>)| -> Box<dyn Evaluable<bool>> { 
                    Box::new( BoolExpr { 
                        t0, 
                        v: v.into_iter().map(|(op, v)| (String::from(op), v)).collect() 
                    })
                }
            )
    )
}

fn precedence1<'a> () -> BoxedParser<'a, Box<dyn Evaluable<bool>>> {
    BoxedParser::new(term())
        .and(
            BoxedParser::new(parse_literals(vec!["||"]))
                .and(term())
                .zero_or_more()
        )
        .map(
            |(t0, v): (Box<dyn Evaluable<bool>>, Vec<(&str, Box<dyn Evaluable<bool>>)>)| -> Box<dyn Evaluable<bool>> { 
                Box::new( BoolExpr { 
                    t0, 
                    v: v.into_iter().map(|(op, v)| (String::from(op), v)).collect() 
                }
            )}
        )
}

fn term<'a> () -> BoxedParser<'a, Box<dyn Evaluable<bool>>> {
    BoxedParser::new(or( 
        map(parse_literals(vec!["true", "false"]), |lit| box_evaluable(lit == "true")),
        map(parse_identifier(), |ident| box_evaluable(ident))
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_bool_expression () {
        let mut env = Environment::new();
        let input1 = "1==2";
        let input2 = "2==2";
        let input3 = "2!=2";
        let input4 = "1*2+3==4+4-3";

        assert!(bool_expression().parse(input1).unwrap().1.eval(&mut env) == false);
        assert!(bool_expression().parse(input2).unwrap().1.eval(&mut env));
        assert!(bool_expression().parse(input3).unwrap().1.eval(&mut env) == false);
        assert!(bool_expression().parse(input4).unwrap().1.eval(&mut env));
    }
}
