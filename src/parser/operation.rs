use std::boxed::Box;

use super::core::*;
use super::*;
use super::expr::expression_op;
use super::assign::assignment_op;
use super::block::block_op;
use crate::ast::*;
use crate::parser::conditional::conditional_if_op;

pub fn operation<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    move |buf: &'a str| {
        if let Ok((buf, out)) = expression_op().parse(buf) {
            Ok((buf, out))
        } else 
        if let Ok((buf, out)) = assignment_op().parse(buf) {
            Ok((buf, out))
        } else
        if let Ok((buf, out)) = block_op().parse(buf) {
            Ok((buf, out))
        } else 
        if let Ok((buf, out)) = conditional_if_op().parse(buf) {
            Ok((buf, out))
        } else
        if let Ok((buf, out)) = string_lit_op().parse(buf) { 
            let bx: Box<dyn Operation> = Box::new( PrintOperation::new(out) );
            Ok((buf, bx))
        } else 
        {
            par_err("no valid operation found.")
        }
    }
}

fn string_lit_op<'a> () -> impl Parser<'a, &'a str> {
    move |buf: &'a str| {
        let parse_joe = parse_literal("joe");
        parse_joe.parse(buf)
    }
}
