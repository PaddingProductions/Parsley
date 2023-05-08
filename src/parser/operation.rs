use std::boxed::Box;

use super::core::*;
use super::*;
use super::expr::*;
use crate::ast::*;

pub fn operation<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    move |buf: &'a str| {
        if let Ok((buf, out)) = map(expression(), expr_to_op).parse(buf) {
            Ok((buf, out))
        } else 

        if let Ok((buf, out)) = string_lit_op().parse(buf) { 
            let bx: Box<dyn Operation> = Box::new( PrintOperation::new(out) );
            Ok((buf, bx))
        } else 

        {
            Err(ParseErr::new("could not find valid operation"))
        }
    }
}

fn string_lit_op<'a> () -> impl Parser<'a, &'a str> {
    move |buf: &'a str| {
        let parse_joe = parse_literal("joe");
        parse_joe.parse(buf)
    }
}
