use crate::parser::{
    *,
    core::*,
};
use crate::ast::{
    Operation,
    Evaluable,
    Expr
};
use crate::interpreter::{Environment, InterpreterErr};



pub struct Declaration {
    pub ident: String, 
    pub eval: Expr 
}


impl Operation for Declaration {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        let val = self.eval.eval(env)?;
        println!("declaring '{}' as {:?}", self.ident, val);
        env.declare(&self.ident, val);
        Ok(())
    }
}


pub fn declaration<'a> (buf: &'a str) -> ParseRes<'a, Declaration> {
    let (buf, _)     = parse_literal("let ").parse(buf)?;
    let (buf, ident) = parse_identifier(buf)?;
    let (buf, _)     = parse_literal("=").parse(buf)?;
    let (buf, eval)  = Expr::any(buf)?;
    Ok((buf, Declaration { ident, eval }))
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Types;

    #[test] 
    fn test_declaration () {
        let mut env = Environment::new();
        let input1 = "let var1 = 1";
        let input2 = "let var2 = 1+2";
        let input3 = "let _var3 = 1*2+3*4+4";

        let declaration = BoxedParser::new(declaration);
        
        declaration.test(input1).exec(&mut env).unwrap();
        declaration.test(input2).exec(&mut env).unwrap();
        declaration.test(input3).exec(&mut env).unwrap();
        
        assert!(*env.test("var1")  == Types::Num(1.0));
        assert!(*env.test("var2")  == Types::Num(3.0));
        assert!(*env.test("_var3") == Types::Num(18.0));
    }
}
