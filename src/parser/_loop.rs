use crate::ast::{ Loop, Operation, Block, Types, Assignment, Evaluable };
use crate::interpreter::{InterpreterErr, Environment};

use super::*;
use super::core::{parse_literal, surround};

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
        .and(expr::expression())
        .and(block::block())
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

        Block { ops: vec, ret_expr: None } 
    };

    BoxedParser::new(parse_literal("for"))
        .and(
            surround("(", ")", 
                assign::assignment().option().suffix(";")
                    .and(expr::expression().suffix(";"))
                    .and(operation::operation().option())
                    .map(|((a, b), c)| (a, b, c))
            ),
        )
        .map(|(_, a)| a)
        .and(block::block())
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
        let mut env = Environment::new();
        let input1 = "cnt=0";
        let input2 = "i = 0";
        let input3 = "while i != 10 { if i % 2 == 0 { cnt = cnt + 1; }; i = i + 1; }";

        operation().parse(input1).unwrap().1.exec(&mut env).unwrap();
        operation().parse(input2).unwrap().1.exec(&mut env).unwrap();
        _while().parse(input3).unwrap().1.exec(&mut env).unwrap();
        
        assert!(*env.vars.get("i").unwrap() == Types::Num(10.0));
        assert!(*env.vars.get("cnt").unwrap() == Types::Num(5.0));
    }

    #[test]
    fn test_for () {
        let mut env = Environment::new();
        let input1 = "cnt = 0";
        let input2 = "for (i=0; i!=10; i=i+1) { if i % 2 == 0 { cnt = cnt + 1; } }";
        let input3 = "d = 0";
        let input4 = "for (; i!=5;) { d = d + i; i = i - 1; }";

        assignment().parse(input1).unwrap().1.exec(&mut env).unwrap();
        _for().parse(input2).unwrap().1.exec(&mut env).unwrap();
        assignment().parse(input3).unwrap().1.exec(&mut env).unwrap();
        _for().parse(input4).unwrap().1.exec(&mut env).unwrap();
        
        assert!(*env.vars.get("cnt").unwrap() == Types::Num(5.0));
        assert!(*env.vars.get("d").unwrap() == Types::Num(40.0));
    }
}
