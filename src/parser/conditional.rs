use super::*;
use super::core::*;
use super::bool_expr::b_expression;
use super::block::block;
use crate::ast::{If, Block, BoolExpression};

impl If {
    pub fn new (bexpr: BoolExpression, block: Block) -> Self {
        Self { bexpr, block }
    }
}

pub fn conditional_if<'a> () -> impl Parser<'a, If> {
    move |buf| -> ParseRes<'a, If> {
        let parse_if = parse_literal("if:");

        let (buf, _) = parse_if.parse(buf)?;
        let (buf, expr) = b_expression().parse(buf)?;
        let (buf, block) = block().parse(buf)?;

        Ok((buf, If::new(expr, block)))
    }
}

use crate::ast::Operation;
pub fn conditional_if_op<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    map(conditional_if(), |_if| -> Box<dyn Operation> {Box::new(_if)} )
}

