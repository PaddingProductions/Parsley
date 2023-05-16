use crate::ast::Types;

pub struct Environment {
    pub vars: std::collections::HashMap<String, Types>
}
impl Environment {
    pub fn new () -> Self {
        Self { vars: std::collections::HashMap::new() }
    }
}

#[derive(Debug)]
pub enum InterpreterErr {
    Err(String),
}
pub fn inter_err (s: &str) -> InterpreterErr {
    InterpreterErr::Err(s.to_owned())
}
