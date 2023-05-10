use super::*;
use super::core::*;
use super::expr::expression;
use crate::ast::{Assignment, Operation, Expression};

pub fn assignment<'a> () -> impl Parser<'a, Assignment> {
    let funct = |(ident, ( _, expr)): (String, (&str, Expression))| Assignment { ident, expr };
    map( 
        and( parse_identifier(), 
        and( parse_literal("="), 
            expression()
        )
        ),
        funct
    )
}

pub fn assignment_op<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    map(assignment(), |o| -> Box<dyn Operation> { Box::new(o) })
}

