use super::*;
use super::core::*;
use crate::ast::Expression;
use crate::interpreter::Environment;

impl Expression {
    pub fn new (t0: f64, v: Vec<(char, f64)>) -> Self {
        Self { t0, v }
    }
}

pub fn expression<'a> () -> impl Parser<'a, Expression> {
    move |buf| -> ParseRes<'a, Expression> {
        let parse_num = parse_number();
        let parse_op = parse_literal("+");

        let (mut buf, t0) = parse_num.parse(buf)?;

        let mut t = 0.0;
        let mut v = vec![];
        while let Ok((mut _buf, op)) = parse_op.parse(buf) {
            (buf, t) = parse_num.parse(_buf)?;
            v.push((op.chars().next().unwrap(), t));
        }

        Ok((buf, Expression::new(t0, v)))
    }
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

