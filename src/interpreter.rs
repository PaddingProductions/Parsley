use crate::ast::*;
use crate::ast::Assignment;

pub struct Environment {
    pub vars: std::collections::HashMap<String, f64>
}
impl Environment {
    pub fn new () -> Self {
        Self { vars: std::collections::HashMap::new() }
    }
}

impl Operation for Assignment {
    fn exec (&self, env: &mut Environment) {
        let val = self.expr.eval(env);
        println!("setting '{}' to {}", self.ident, val);
        env.vars.insert(self.ident.clone(), val);
    }
}



impl Operation for Block {
    fn exec (&self, env: &mut Environment) {
        for op in self.ops.iter() {
            op.exec(env);
        }
    }
}

/*
impl Operation for If {
    fn exec (&self, env: &mut Environment) {
        if self.expr.eval(env) {
            println!("conditional boolean expression evaluated to true, running block");
            self.block.exec(env);
        }
    }
}
*/
