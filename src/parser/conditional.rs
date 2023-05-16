use super::*;
use super::core::*;
use super::block::block;
use super::expr::expression;
use crate::ast::{If, Operation, Types};
use crate::interpreter::{Environment, InterpreterErr, inter_err};


impl Operation for If {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        if let Types::Bool(b) = self.expr.eval(env)? {
            if b {
                return self.block.exec(env);
            } else {
                Ok(())
            }
        } else {
            return Err(inter_err("'if' expression does not evaluate to Bool()")); // Expression does not evaluate to a boolean
        }
    }
}
pub fn conditional_if<'a> () -> BoxedParser<'a, If> {
    BoxedParser::new(parse_literal("if "))
        .and(expression())
        .and(block())
        .map(
            |((_, expr), block)| If { expr, block }
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_conditional () {
        let mut env = Environment::new();
        let input1 = "{res1=0;res2=0;}";
        let input2 = "if 1+2!=3{res1=1;}";
        let input3 = "if 1*3==4*2-5{res2=2;}";
        let input4 = "if true!=false{res2=3;}";


        block().parse(input1).unwrap().1.exec(&mut env).unwrap();
        conditional_if().parse(input2).unwrap().1.exec(&mut env).unwrap();
        conditional_if().parse(input3).unwrap().1.exec(&mut env).unwrap();
        conditional_if().parse(input4).unwrap().1.exec(&mut env).unwrap();
        assert!(*env.vars.get("res1").unwrap() == Types::Num(0.0));
        assert!(*env.vars.get("res2").unwrap() == Types::Num(3.0));
    }
}
