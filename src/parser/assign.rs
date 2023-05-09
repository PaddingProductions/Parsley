use super::*;
use super::core::*;
use super::expr::expression;
use crate::ast::{Assignment, Operation};

pub fn assignment<'a> () -> impl Parser<'a, Assignment> {
    move |buf: &'a str| -> ParseRes<'a, Assignment> {
        let (buf, ident) = parse_identifier().parse(buf)?;
        let (buf, _)     = parse_literal("=").parse(buf)?;
        let (buf, expr)  = expression().parse(buf)?;
        Ok((buf, Assignment {
            ident,
            expr
        }))
    }
}

pub fn assignment_op<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    map(assignment(), |o| -> Box<dyn Operation> { Box::new(o) })
}

