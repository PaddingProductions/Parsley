use std::io::{self, BufRead};

mod ast;
mod parser;
mod interpreter;

use parser::Parser;
use parser::operation::operation;

fn main () {
    let mut stdin = io::stdin().lock();

    loop {
        let mut buf = String::with_capacity(100);
        stdin.read_line(&mut buf).expect("STDIN failed");
        let s: &str = buf.as_str();
        if let Ok((_, op)) = operation().parse(s) {
            op.exec();
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
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
        op.unwrap().1.exec();
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
        let input = "1+2";
        let op = operation().parse(input); 
        op.unwrap().1.exec();
    }
}
