use super::*;
use super::core::*;
use super::expr::expression;
use crate::interpreter::{Environment, InterpreterErr};
use crate::ast::*;

impl Operation for Assignment {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        let val = self.expr.eval(env)?;
        println!("setting '{}' to {:?}", self.ident, val);
        env.vars.insert(self.ident.clone(), val);
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

        assignment().parse(input1).unwrap().1.exec(&mut env).unwrap();
        assignment().parse(input2).unwrap().1.exec(&mut env).unwrap();
        assignment().parse(input3).unwrap().1.exec(&mut env).unwrap();
        
        assert!(*env.vars.get("var1").unwrap()   == Types::Num(1.0));
        assert!(*env.vars.get("var2").unwrap()   == Types::Num(3.0));
        assert!(*env.vars.get("_var3").unwrap()  == Types::Num(18.0));
    }
}
