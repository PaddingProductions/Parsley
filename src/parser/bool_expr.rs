use super::*;
use super::core::*;
use super::expr::expression;
use crate::ast::{Expression, BoolExpression};
use crate::interpreter::Environment;

impl BoolExpression {
    pub fn new (a: Expression, b: Expression, op: String) -> Self {
        Self{ a, b, op }
    }
}

pub fn b_expression<'a> () -> impl Parser<'a, BoolExpression> {
    move |buf| -> ParseRes<'a, BoolExpression> {
        let parse_eq = parse_literal("==");

        let (buf, a) = expression().parse(buf)?;
        let (buf, op) = parse_eq.parse(buf)?;
        let (buf, b) = expression().parse(buf)?;

        Ok((buf, BoolExpression::new(a, b, String::from(op))))
    }
}

pub struct BoolExprOp {
    bexpr: BoolExpression
}
impl Operation for BoolExprOp {
    fn exec(&self, env: &mut Environment) {
        println!("boolean expression evaluated to: {}", self.bexpr.eval());
    }
}

use crate::ast::Operation;
pub fn b_expression_op<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    map(b_expression(), |bexpr| -> Box<dyn Operation> { Box::new( BoolExprOp{ bexpr }) })
}

