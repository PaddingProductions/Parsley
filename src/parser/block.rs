use super::*;
use super::core::*;
use crate::ast::{Operation, Block};
use super::operation::operation;

impl Block {
    pub fn new (ops: Vec<Box<dyn Operation>>) -> Self {
        Self { ops } 
    }
}

pub fn block<'a> () -> impl Parser<'a, Block> {
    surround(
        "{", "}",
        map (
            zero_or_more(
                and( 
                    operation(),
                    parse_literal(";")
                )
            ),
            |v| Block::new( v.into_iter().map(|(op, _)| op).collect() )
        )
    )

    /*
    |buf: &'a str| {
        let parse_semi = parse_literal(";"); 
        let parse_open = parse_literal("{"); 
        let parse_close = parse_literal("}"); 

        let mut v = vec![];
        let mut buf = buf;

        (buf, _) = parse_open.parse(buf)?;
        loop {
            match parse_close.parse(buf) {
                Err(_) => { 
                    println!("{}", buf);
                    let _buf = buf;
                    let (_buf, op) = operation().parse(_buf)?;
                    let (_buf, _) = parse_semi.parse(_buf)?;

                    v.push(op);
                    buf = _buf;
                }, 
                Ok((_buf, _)) => {
                    buf = _buf;
                    break;
                }
            }
        }
        Ok((buf, Block::new(v)))
    }
    */
}

pub fn block_op<'a> () -> impl Parser<'a, Box<dyn Operation>> {
    map(block(), |o| -> Box<dyn Operation> { Box::new(o) })
}

