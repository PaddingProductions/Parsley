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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn expr () {
        let mut env = Environment::new();
        let input = "{var1=1;var2=1+2;_var3=1*2+3*4+4;}";

        block().parse(input).unwrap().1.exec(&mut env);
        
        assert!(env.vars.get("var1").unwrap()   == &1.0);
        assert!(env.vars.get("var2").unwrap()   == &3.0);
        assert!(env.vars.get("_var3").unwrap()  == &18.0);
    }
}
