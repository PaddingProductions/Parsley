use super::{*, expr::expression};
use super::core::*;
use crate::ast::*;
use super::operation::operation;
use crate::interpreter::{Environment, InterpreterErr};

impl Evaluable for Block {
    fn eval (&self, env: &mut Environment) -> Result<Types, InterpreterErr> {
        for op in self.ops.iter() {
            op.exec(env)?;
        }
        println!("{:?}", self.ret_expr.is_some());
        let out = if let Some(expr) = self.ret_expr.as_ref() { expr.eval(env)? } else { Types::Nil };
        Ok(out)       
    }
}

pub fn block<'a> () -> BoxedParser<'a, Block> {
    BoxedParser::new( surround( "{", "}",
        operation()
            .and( parse_literals(vec![";", "\n"]) )
            .zero_or_more()
            .map(
                |v| v.into_iter().map(|(op, _)| op).collect()
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
    fn test_block () {
        let mut env = Environment::new();
        let input = "{var1=1;var2=1+2;_var3=1*2+3*4+4;}";

        block().parse(input).unwrap().1.exec(&mut env).unwrap();
        
        assert!(*env.vars.get("var1").unwrap()   == Types::Num(1.0));
        assert!(*env.vars.get("var2").unwrap()   == Types::Num(3.0));
        assert!(*env.vars.get("_var3").unwrap()  == Types::Num(18.0));
    }

    use super::assign::assignment;
    #[test]
    fn test_block_return () {
        let mut env = Environment::new();
        let input1 = "a = { b = 10; b - 9 } + 10";
        let input2 = "c = 2 * { a - 9 }";

        assignment().parse(input1).unwrap().1.exec(&mut env).unwrap();
        assignment().parse(input2).unwrap().1.exec(&mut env).unwrap();
        
        assert!(*env.vars.get("a").unwrap() == Types::Num(11.0));
        assert!(*env.vars.get("c").unwrap() == Types::Num(4.0));
    }
}
