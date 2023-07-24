use crate::ast::{
    Operation, 
    Block, 
    Types, 
    Expr,
    Evaluable,
};
use crate::interpreter::{InterpreterErr, Environment};
use crate::parser::{
    *,
    core::{parse_literal, surround},
};



pub struct Loop {
    pub expr: Expr,
    pub block: Block,
}


impl Operation for Loop {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        while let Types::Bool(b) = self.expr.eval(env)? {
            if !b { break }
            self.block.exec(env)?;
        }
        Ok(())
    }
}


pub fn _while<'a> () -> BoxedParser<'a, Loop> {
    BoxedParser::new(parse_literal("while "))
        .and(Expr::bool)
        .and(block::block)
        .map(|((_, expr), block)| Loop{ expr, block })
}


pub fn _for<'a> () -> BoxedParser<'a, Block> {
    let func = |((assign, expr, op), mut block): (_, Block)| -> Block { 
        if let Some(op) = op { block.ops.push(op) };

        let lop = Loop { expr, block };

        let vec: Vec<Box<dyn Operation>> = 
            if let Some(assign) = assign { 
                vec![Box::new(assign), Box::new(lop)] 
            } else {
                vec![Box::new(lop)]
            };

        Block { ops: vec } 
    };

    BoxedParser::new(parse_literal("for"))
        .and(
            surround("(", ")", 
                BoxedParser::new(declare::declaration).option().suffix(";")
                    .and(suffix(";", Expr::bool))
                    .and(option(operation::operation))
                    .map(|((a, b), c)| (a, b, c))
            ),
        )
        .map(|(_, a)| a)
        .and(block::block)
        .map(func)
}




#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::Types;
    use crate::parser::assign::assignment;
    use crate::parser::operation::operation;
    use crate::interpreter::Environment;

    #[test]
    fn test_while () {
        /* Cannot be used until equality is implemented
        let mut env = Environment::new();
        let input1 = "let cnt=0";
        let input2 = "let i = 0";
        let input3 = 
            "while i != 10 { 
                if i % 2 == 0 { 
                    cnt = cnt + 1 
                }
                i = i + 1 
            }";

        operation().test(input1).exec(&mut env).unwrap();
        operation().test(input2).exec(&mut env).unwrap();
        _while().test(input3).exec(&mut env).unwrap();
        
        assert!(*env.test("i")   == Types::Num(10.0));
        assert!(*env.test("cnt") == Types::Num(5.0));
        */
    }

    #[test]
    fn test_for () {
        /* Cannot be used until equality is implemented
        let mut env = Environment::new();
        let input1 = "let cnt = 0";
        let input2 = "for (let i=0; i!=10; i=i+1) { if i % 2 == 0 { cnt = cnt + 1 } }";
        let input3 = "let d = 0";
        let input4 = 
            "for (; i!=5;) { 
                d = d + i 
                i = i - 1 
            }";

        assignment().test(input1).exec(&mut env).unwrap();
        _for().test(input2).exec(&mut env).unwrap();

        assignment().test(input3).exec(&mut env).unwrap();
        _for().test(input4).exec(&mut env).unwrap();
        
        assert!(*env.test("cnt") == Types::Num(5.0));
        assert!(*env.test("d")   == Types::Num(40.0));
        */
    }
}
