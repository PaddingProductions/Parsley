use std::boxed::Box;

use super::*;
use crate::ast::*;

pub fn operation<'a> (buf: &'a str) -> ParseRes<'a, Box<dyn Operation>> {
    // If conditional
    map(block::block, box_operation).parse(buf)
    .or_else( 
        |_| _loop::_for() 
            .map(box_operation)
            .parse(buf)
    )
    .or_else(
        |_| _loop::_while()
            .map(box_operation)
            .parse(buf)
    )
    .or_else(
        |_| map(conditional::conditional_if, box_operation)
            .parse(buf)
    )
    .or_else(
        |_| print::print()
            .parse(buf)
    )
    .or_else(
        |_| map(declare::declaration, box_operation).parse(buf)
    )
    .or_else(
        |_| map(assign::assignment(), box_operation).parse(buf) 
    )
}

fn box_operation<T> (o: T) -> Box<dyn Operation>
where
    T: Operation + 'static
{
    let b: Box<dyn Operation> = Box::new(o);
    b
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Environment;

    #[test] 
    fn test_operation () {
        let mut env = Environment::new();
        let input1 = "let var1 = 1";
        let input2 = "let var2 = 1+1";
        let input3 = "var2 = var2+var1*3";
        let input4 = "if true { var1 = 0 }";

        let parser = BoxedParser::new(operation);
        parser.test(input1).exec(&mut env).unwrap();
        parser.test(input2).exec(&mut env).unwrap();
        parser.test(input3).exec(&mut env).unwrap();
        parser.test(input4).exec(&mut env).unwrap();
        
        assert!(*env.test("var1") == Types::Num(0.0));
        assert!(*env.test("var2") == Types::Num(5.0));
    }
}
