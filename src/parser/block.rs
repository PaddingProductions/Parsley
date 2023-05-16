use super::*;
use super::core::*;
use crate::ast::*;
use super::operation::operation;
use crate::interpreter::{Environment, InterpreterErr};

impl Block {
    pub fn new (ops: Vec<Box<dyn Operation>>) -> Self {
        Self { ops } 
    }
}
impl Operation for Block {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        for op in self.ops.iter() {
            op.exec(env)?;
        }
        Ok(())
    }
}
pub fn block<'a> () -> BoxedParser<'a, Block> {
    BoxedParser::new( surround( "{", "}",
        operation()
            .and( parse_literals(vec![";", "\n"]) )
            .zero_or_more()
            .map(
                |v| Block::new( v.into_iter().map(|(op, _)| op).collect() )
            )
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_block () {
        let mut env = Environment::new();
        let input = "{var1=1;var2=1+2;_var3=1*2+3*4+4;}";

        block().parse(input).unwrap().1.exec(&mut env).unwrap();
        
        assert!(*env.vars.get("var1").unwrap()   == Types::Num(1.0));
        assert!(*env.vars.get("var2").unwrap()   == Types::Num(3.0));
        assert!(*env.vars.get("_var3").unwrap()  == Types::Num(18.0));
    }
}
