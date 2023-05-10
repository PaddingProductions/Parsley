use super::*;
use super::core::*;
use crate::ast::Expression;
use crate::interpreter::Environment;

impl Expression {
    pub fn new (t0: f64, v: Vec<(String, f64)>) -> Self {
        Self { t0, v }
    }
}

pub fn expression<'a> () -> impl Parser<'a, Expression> {
    map (
        and(
            parse_number(),
            zero_or_more(
                and(
                    parse_literals(vec!["+", "-", "*", "/"]),
                    parse_number()
                )
            )
        ),
        |(t0, v)| Expression::new(t0, v.into_iter().map(|(op, v)| (String::from(op), v.clone())).collect())
    )
}

pub struct ExprOp {
    expr: Expression
}
impl Operation for ExprOp {
    fn exec(&self, env: &mut Environment) {
        println!("expression evaluated to: {}", self.expr.eval());
    }
}

use crate::ast::Operation;
pub fn expression_op<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    map(expression(), |expr| -> Box<dyn Operation> { Box::new( ExprOp{ expr }) })
}

