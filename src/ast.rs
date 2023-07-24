use crate::interpreter::{Environment, InterpreterErr};


pub use crate::parser::expr::Expr; 
pub use crate::parser::expr::Evaluable; 
pub use crate::parser::declare::Declaration; 
pub use crate::parser::assign::Assignment; 
pub use crate::parser::block::Block; 
pub use crate::parser::conditional::If; 
pub use crate::parser::_loop::Loop; 



impl Evaluable for Types {
    fn eval (&self, _env: &mut Environment) -> Result<Types, InterpreterErr> { Ok(self.clone()) } 
}


impl<T: Evaluable> Operation for T {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr> {
        self.eval(env)?;
        Ok(())
    }
}


pub trait Operation {
    fn exec (&self, env: &mut Environment) -> Result<(), InterpreterErr>;
}


#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Nil,
    Num (f64),
    Bool (bool)
}
