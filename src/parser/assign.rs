use crate::parser::{
    *,
    core::*
};
use crate::ast::{
    Operation,
    Evaluable,
    Expr
};
use crate::interpreter::{Environment, InterpreterErr};



pub struct Assignment {
    pub ident: String, 
    pub expr: Expr
}


impl Operation for Assignment {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        let val = self.expr.eval(env)?;
        println!("setting '{}' to {:?}", self.ident, val);
        env.set(&self.ident, val)?;
        Ok(())
    }
}


pub fn assignment<'a> () -> BoxedParser<'a, Assignment> {
    BoxedParser::new(parse_identifier) 
        .and(parse_literal("="))
        .and(Expr::any)
        .map( 
            |((ident,  _), expr)| Assignment { ident, expr }
        )
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::Types, parser::declare::declaration};

    #[test] 
    fn test_assignment () {
        let mut env = Environment::new();
        let input1 = "let var1=1";
        let input2 = "var1=1+2";

        let declaration = BoxedParser::new(declaration);
        declaration.test(input1).exec(&mut env).unwrap();
        assignment().test(input2).exec(&mut env).unwrap();
        
        assert!(*env.test("var1")  == Types::Num(3.0));
    }
}
