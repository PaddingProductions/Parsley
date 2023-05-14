use std::boxed::Box;

use super::*;
use super::assign::assignment;
use super::block::block;
use super::conditional::conditional_if;
use super::print::print;
use crate::ast::*;
//use crate::parser::conditional::conditional_if;

pub fn operation<'a> () -> BoxedParser<'a, Box<dyn Operation>> {
    print()
        .or(assignment().map(box_operation))
        .or(block().map(box_operation))
        .or(conditional_if().map(box_operation))
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
        let input2 = "var2=1+2*8";
        let input3 = "var3=var2+var1";

        operation().parse(input1).unwrap().1.exec(&mut env);
        operation().parse(input2).unwrap().1.exec(&mut env);
        operation().parse(input3).unwrap().1.exec(&mut env);
        
        assert!(env.vars.get("var1").unwrap()   == &1.0);
        assert!(env.vars.get("var2").unwrap()   == &17.0);
        assert!(env.vars.get("var3").unwrap()   == &18.0);
    }
}
