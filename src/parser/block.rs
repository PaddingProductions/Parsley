use std::sync::OnceLock;

use crate::parser::{
    *,
    core::*,
};
use crate::ast::Operation;
use super::operation::operation;
use crate::interpreter::{Environment, InterpRes};


pub struct Block {
    pub ops: Vec<Box<dyn Operation>>,
} 


impl Block {
    pub fn empty () -> Self {
        Block { ops: vec![] }
    }
} 


impl Operation for Block {
    fn exec (&self, env: &mut Environment) -> InterpRes<()> {
        
        // Spawn scope
        env.new_scope();

        for op in self.ops.iter() {
            op.exec(env)?;
        }

        // Exit scope
        env.exit_scope();
        Ok(())
    }
}

static cell: OnceLock<BoxedParser<Block>> = OnceLock::new();
pub fn block<'a> (buf: &'a str) -> ParseRes<'a, Block> {
    cell.get_or_init( 
        || BoxedParser::new( prefix("{", zero_or_more(operation)) )
            .map( |v| v.into_iter().collect() )
            .and( parse_literal("}") )
            .map( |(o, _)| o )
            .map( |v| Block { ops: v } )
    ).parse(buf)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Types;
    use crate::interpreter::{ 
        Environment,
        InterpreterErr
    };

    #[test] 
    fn test_block () {
        let mut env = Environment::new();
        let input1 = "let var1 = 0";
        let input2 = "{
          let var1 = 1\n 
          let var3 = 10
        }";

        let operation = BoxedParser::new(operation);
        let block = BoxedParser::new(block);

        operation.test(input1).exec(&mut env).unwrap();
        block.test(input2).exec(&mut env).unwrap();
        
        assert!(*env.test("var1") == Types::Num(0.0));
        let v3 = env.var(&"var3".to_owned());
        assert!(matches!(Result::<&Types, InterpreterErr>::Err, v3));
    }
}
