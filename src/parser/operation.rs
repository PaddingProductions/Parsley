use std::boxed::Box;

use super::*;
use crate::ast::*;

pub fn operation<'a> () -> BoxedParser<'a, Box<dyn Operation>> {
   BoxedParser::new( |buf: &'a str| {
        // If conditional
        if let Ok(_) = parse_literal("{").parse(buf) {
            block::block()
                .map(box_operation)
                .parse(buf)
        } else  
        if let Ok(_) = parse_literal("for").parse(buf) {
            _loop::_for()
                .map(box_operation)
                .parse(buf)
        } else 
        if let Ok(_) = parse_literal("while").parse(buf) {
            _loop::_while()
                .map(box_operation)
                .parse(buf)
        } else 
        if let Ok(_) = parse_literal("if").parse(buf) {
            conditional::conditional_if()
                .map(box_operation)
                .parse(buf)
        } else 
        if let Ok(_) = parse_literal("print").parse(buf) {
            print::print()
                .parse(buf)
        } else 
        if let Ok((buf, out)) = assign::assignment().parse(buf) {
            Ok((buf, box_operation(out)))
        } else
        {
            par_err(buf, "no valid operation found.")
        }
    })
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
        let input1 = "{var1=1;var2=1+1;var3=3;}";
        let input2 = "var2=var2+var1*var3";
        let input3 = "var3=var2+var1";

        operation().parse(input1).unwrap().1.exec(&mut env).unwrap();
        operation().parse(input2).unwrap().1.exec(&mut env).unwrap();
        operation().parse(input3).unwrap().1.exec(&mut env).unwrap();
        
        assert!(*env.vars.get("var1").unwrap() == Types::Num(1.0));
        assert!(*env.vars.get("var2").unwrap() == Types::Num(5.0));
        assert!(*env.vars.get("var3").unwrap() == Types::Num(6.0));
    }
}
