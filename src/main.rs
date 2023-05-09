use std::io::{self, BufRead};

mod ast;
mod parser;
mod interpreter;

use interpreter::Environment;
use parser::Parser;
use parser::operation::operation;

fn main () {
    let mut stdin = io::stdin().lock();
    let mut env = Environment::new();

    loop {
        let mut buf = String::with_capacity(100);
        stdin.read_line(&mut buf).expect("STDIN failed");
        let s: &str = buf.as_str();
        if let Ok((_, op)) = operation().parse(s) {
            op.exec(&mut env);
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Environment;
    use crate::parser::*;
    use crate::parser::bool_expr::b_expression;
    use crate::parser::core::parse_literal;


    #[test]
    fn test_parse_literal () {
        let input = "joemama";
        let parse_joe = parse_literal("joe");
        assert_eq!(Ok(("mama", "joe")), parse_joe.parse(input));
    }

    use crate::parser::operation::operation;
    #[test]
    fn test_operation () {
        let input = "joe biden";
        let op = operation().parse(input); 
        let mut env = Environment::new();
        op.unwrap().1.exec(&mut env);
    }

    use crate::ast::Expression;
    use crate::parser::expr::expression;
    #[test]
    fn test_expression_parse () {
        let input = "1+2";
        let expr = expression().parse(input); 
        assert_eq!(Ok(("", Expression { t0: 1.0, v: vec![('+', 2.0)] })), expr);
    }

    #[test]
    fn test_expression_operation () {
        let mut env = Environment::new();
        let input = "1";
        operation().parse(input).unwrap().1.exec(&mut env);

        let input = "1+2";
        operation().parse(input).unwrap().1.exec(&mut env);
    }

    #[test]
    fn test_assignment () {
        let mut env = Environment::new();
        let input = "varA=12";
        operation().parse(input).unwrap().1.exec(&mut env);
        let input = "var3=3";
        operation().parse(input).unwrap().1.exec(&mut env);
        let input = "varA=4+3";
        operation().parse(input).unwrap().1.exec(&mut env);

        assert!(*env.vars.get("varA").unwrap() == 7.0);
        assert!(*env.vars.get("var3").unwrap() == 3.0);
    }
    
    #[test]
    fn test_block () {
        let mut env = Environment::new();
        let input = "{var1=1;var2=2;}";

        operation().parse(input).unwrap().1.exec(&mut env);
        
        assert!(*env.vars.get("var1").unwrap() == 1.0);
        assert!(*env.vars.get("var2").unwrap() == 2.0);
    }

    #[test]
    fn test_b_expr () {
        let input = "1==1";
        assert!(b_expression().parse(input).unwrap().1.eval());
        let input = "1==10";
        assert!(!b_expression().parse(input).unwrap().1.eval());
        let input = "2+8==10";
        assert!(b_expression().parse(input).unwrap().1.eval());
    }

    #[test]
    fn test_if () {
        let mut env = Environment::new();
        let input = "{var1=1+2;if:1+2==3{var1=2;};}";

        operation().parse(input).unwrap().1.exec(&mut env);
        assert!(*env.vars.get("var1").unwrap() == 2.0);
    }
}
