use std::sync::OnceLock;

use crate::parser::{
    *,
    core::*,
    expr::Expr,
};
use crate::ast::{Types, Evaluable, Operation, Block};
use crate::interpreter::{Environment, inter_err, InterpRes};


pub struct If {
    pub expr: Expr,
    pub block: Block, 
    pub else_block: Block,
}


impl Operation for If {
    fn exec (&self, env: &mut Environment) -> InterpRes<()> {
        if let Types::Bool(b) = self.expr.eval(env)? {
            if b {
                self.block.exec(env)?;
            } else {
                self.else_block.exec(env)?;
            }
            Ok(())
        } else {
            inter_err("'if' expression does not evaluate to Bool()") // Expression does not evaluate to a boolean
        }
    }
}

static cell: OnceLock<BoxedParser<If>> = OnceLock::new();
pub fn conditional_if<'a> (buf: &'a str) -> ParseRes<'a, If> {
    cell.get_or_init( ||     
        BoxedParser::new(parse_literal("if "))
            .and(Expr::bool)
            .and(block::block)
            .map(|((_, expr), block)| (expr, block))
            .and(
                BoxedParser::new(parse_literal("else"))
                    .and(block::block)
                    .map(|(_, block)| block)
                    .option_with_default(&|| Block::empty())
            )
            .map(
                |((expr, block), else_block)| If { expr, block, else_block }
            )
    ).parse(buf)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{interpreter::Environment, ast::Operation, parser::declare::declaration};

    #[test] 
    fn test_conditional () {
        let mut env = Environment::new();
        let input1 = "let res1 = 0";
        let input2 = "let res2 = 0";
        let input3 = "if true&&true { res1=1}";
        let input4 = "if false||true{res2=2}";
        let input5 = "if false||false {res2=3}";

        let declaration = BoxedParser::new(declaration);
        let conditional_if = BoxedParser::new(conditional_if);

        declaration.test(input1).exec(&mut env).unwrap();
        declaration.test(input2).exec(&mut env).unwrap();

        conditional_if.test(input3).exec(&mut env).unwrap();
        conditional_if.test(input4).exec(&mut env).unwrap();
        conditional_if.test(input5).exec(&mut env).unwrap();

        println!("{:?}", *env.test("res1"));
        assert!(*env.test("res1") == Types::Num(1.0));
        assert!(*env.test("res2") == Types::Num(2.0));
    }
}
