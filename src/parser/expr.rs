use super::*;
use super::core::*;
use crate::ast::Expression;

pub fn expression<'a> () -> impl Parser<'a, Expression> {
    move |buf| -> ParseRes<'a, Expression> {
        let parse_num = parse_number();
        let parse_op = parse_literal("+");

        let (mut buf, t0) = parse_num.parse(buf)?;

        let mut t = 0.0;
        let mut v = vec![];
        while let Ok((mut _buf, op)) = parse_op.parse(buf) {
            (_buf, t) = parse_num.parse(_buf)?;
            v.push((op.chars().next().unwrap(), t));

            buf = _buf;
        }

        Ok((buf, Expression::new(t0, v)))
    }
}

pub struct ExpressionOperation {
    expr: Expression
}
impl Operation for ExpressionOperation {
    fn exec(&self) {
        println!("expression evaluated to: {}", self.expr.eval());
    }
}

use crate::ast::Operation;
pub fn expr_to_op (expr: Expression) -> Box<dyn Operation> {
    Box::new(ExpressionOperation { expr })
}
