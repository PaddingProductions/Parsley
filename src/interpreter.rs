use crate::ast::Types;

pub struct Environment {
    pub vars: std::collections::HashMap<String, Types>
}
impl Environment {
    pub fn new () -> Self {
        Self { vars: std::collections::HashMap::new() }
    }
}
