use super::{*, expr::expression};
use super::core::*;
use crate::ast::*;
use super::operation::operation;
use crate::interpreter::{Environment, InterpreterErr};

impl Evaluable for Block {
    fn eval (&self, env: &mut Environment) -> Result<Types, InterpreterErr> {
        
        // Spawn scope
        env.new_scope();

        for op in self.ops.iter() {
            op.exec(env)?;
        }

        println!("{:?}", self.ret_expr.is_some());
        let out = if let Some(expr) = self.ret_expr.as_ref() { expr.eval(env)? } else { Types::Nil };

        // Exit scope
        env.exit_scope();

        Ok(out)       
    }
}

pub fn block<'a> () -> BoxedParser<'a, Block> {
    BoxedParser::new( surround( "{", "}",
        operation()
            .zero_or_more()
            .map(
                |v| v.into_iter().collect()
            )
            .and( expression().option() )
            .map(|(v, ret_expr)| Block { ops: v, ret_expr })
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_block_scope () {
        let mut env = Environment::new();
        let input1 = "var1 = 0;";
        let input2 = "
        {
          var1=1;\n 
          var2=1+2;
          _var3=1*2+3*4+4;
        }";

        operation().test(input1).exec(&mut env).unwrap();
        block().test(input2).exec(&mut env).unwrap();
        
        assert!(*env.test("var1") == Types::Num(0.0));

        let v2 = env.var(&"var2".to_owned());
        let v3 = env.var(&"_var3".to_owned());
        assert!(matches!(Result::<&Types, InterpreterErr>::Err, v2));
        assert!(matches!(Result::<&Types, InterpreterErr>::Err, v3));
    }

    use super::assign::assignment;
    #[test]
    fn test_block_return () {
        let mut env = Environment::new();
        let input1 = "a = { b = 10; b - 9 } + 10";
        let input2 = "c = 2 * { a - 9 }";

        assignment().test(input1).exec(&mut env).unwrap();
        assignment().test(input2).exec(&mut env).unwrap();
        
        assert!(*env.test("a") == Types::Num(11.0));
        assert!(*env.test("c") == Types::Num(4.0));
    }
}
