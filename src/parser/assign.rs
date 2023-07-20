use super::*;
use super::core::*;
use super::expr::expression;
use crate::interpreter::{Environment, InterpreterErr};
use crate::ast::*;

impl Operation for Assignment {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        let val = self.expr.eval(env)?;
        println!("setting '{}' to {:?}", self.ident, val);
        env.set(&self.ident, val);
        Ok(())
    }
}

pub fn assignment<'a> () -> BoxedParser<'a, Assignment> {
    BoxedParser::new(parse_identifier()) 
        .and(parse_literal("="))
        .and(expression())
        .map( 
            |((ident,  _), expr)| Assignment { ident, expr }
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_assignment () {
        let mut env = Environment::new();
        let input1 = "var1=1";
        let input2 = "var2=1+2";
        let input3 = "_var3=1*2+3*4+4";

        assignment().test(input1).exec(&mut env).unwrap();
        assignment().test(input2).exec(&mut env).unwrap();
        assignment().test(input3).exec(&mut env).unwrap();
        
        assert!(*env.test("var1")  == Types::Num(1.0));
        assert!(*env.test("var2")  == Types::Num(3.0));
        assert!(*env.test("_var3") == Types::Num(18.0));
    }
}
