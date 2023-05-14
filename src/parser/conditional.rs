use super::*;
use super::core::*;
use super::bool_expr::bool_expression;
use super::block::block;
use crate::ast::{If, Operation};
use crate::interpreter::Environment;


impl Operation for If {
    fn exec (&self, env: &mut Environment) {
        if self.expr.eval(env) {
            self.block.exec(env);
        }
    }
}
pub fn conditional_if<'a> () -> BoxedParser<'a, If> {
    BoxedParser::new(parse_literal("if "))
        .and(bool_expression())
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
        let input2 = "if:1+2!=3{res1=1;}";
        let input3 = "if:1*3==4*2-5{res2=2;}";
        let input4 = "if:true!=false{res2=3;}";


        block().parse(input1).unwrap().1.exec(&mut env);
        conditional_if().parse(input2).unwrap().1.exec(&mut env);
        conditional_if().parse(input3).unwrap().1.exec(&mut env);
        conditional_if().parse(input4).unwrap().1.exec(&mut env);
        assert!(*env.vars.get("res1").unwrap() == 0.0);
        assert!(*env.vars.get("res2").unwrap() == 3.0);
    }
}
