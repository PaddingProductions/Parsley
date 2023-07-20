use super::*;
use super::core::*;
use super::block::block;
use super::expr::expression;
use crate::ast::{If, Types, Evaluable, Block};
use crate::interpreter::{Environment, InterpreterErr, inter_err};


impl Evaluable for If {
    fn eval (&self, env: &mut Environment) -> Result<Types, InterpreterErr> {
        if let Types::Bool(b) = self.expr.eval(env)? {
            if b {
                return self.block.eval(env);
            } else {
                return self.else_block.eval(env);
            }
        } else {
            return inter_err("'if' expression does not evaluate to Bool()") // Expression does not evaluate to a boolean
        }
    }
}
pub fn conditional_if<'a> () -> BoxedParser<'a, If> {
    BoxedParser::new(parse_literal("if "))
        .and(expression())
        .and(block())
        .map(|((_, expr), block)| (expr, block))
        .and(
            BoxedParser::new(parse_literal("else"))
                .and(block())
                .map(|(_, block)| block)
                .option_with_default(&|| Block::empty())
        )
        .map(
            |((expr, block), else_block)| If { expr, block, else_block }
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{interpreter::Environment, ast::Operation, parser::assign::assignment};

    #[test] 
    fn test_conditional () {
        let mut env = Environment::new();
        let input1 = "{res1=0;res2=0;}";
        let input2 = "if 1+2!=3{res1=1;}";
        let input3 = "if 1*3==4*2-5{res2=2;}";
        let input4 = "if true!=false{res2=3;}";

        block().test(input1).eval(&mut env).unwrap();
        conditional_if().test(input2).exec(&mut env).unwrap();
        conditional_if().test(input3).exec(&mut env).unwrap();
        conditional_if().test(input4).exec(&mut env).unwrap();

        assert!(*env.test("res1") == Types::Num(0.0));
        assert!(*env.test("res2") == Types::Num(3.0));
    }

    #[test]
    fn test_conditional_in_expression () {
        let mut env = Environment::new();
        let input1 = "{a=0; b=0;}";
        let input2 = "c = if a == b { b + 2 } * 2";

        block().test(input1).eval(&mut env).unwrap();
        assignment().test(input2).exec(&mut env).unwrap();
        assert!(*env.test("c") == Types::Num(4.0));
    }
}
