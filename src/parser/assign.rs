use super::*;
use super::core::*;
use super::expr::expression;
use crate::interpreter::Environment;
use crate::ast::{Assignment, Operation};

pub fn assignment<'a> () -> impl Parser<'a, Assignment> {
    let funct = |(ident, ( _, expr))| Assignment { ident, expr };
    map( 
        and( parse_identifier(), 
        and( parse_literal("="), 
            expression()
        )
        ),
        funct
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

        assignment().parse(input1).unwrap().1.exec(&mut env);
        assignment().parse(input2).unwrap().1.exec(&mut env);
        assignment().parse(input3).unwrap().1.exec(&mut env);
        
        assert!(env.vars.get("var1").unwrap()   == &1.0);
        assert!(env.vars.get("var2").unwrap()   == &3.0);
        assert!(env.vars.get("_var3").unwrap()  == &18.0);
    }
}
