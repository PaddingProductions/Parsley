use crate::ast::*;

pub struct Environment {
    pub vars: std::collections::HashMap<String, f64>
}
impl Environment {
    pub fn new () -> Self {
        Self { vars: std::collections::HashMap::new() }
    }
}

impl Operation for Block {
    fn exec (&self, env: &mut Environment) {
        for op in self.ops.iter() {
            op.exec(env);
        }
    }
}

impl Expression {
    pub fn eval (&self) -> f64 {
        let mut res = self.t0;
        for (op, v) in self.v.iter() {
            match op.as_str() {
                "+" => res += v,
                "*" => res *= v,
                "-" => res -= v,
                "/" => res /= v,
                _ => panic!("invalid operator character found in expression")
            }
        }
        res
    }
}
impl BoolExpression {
    pub fn eval (&self) -> bool {
        let a = self.a.eval();
        let b = self.b.eval();
        match self.op.as_str() {
            "==" => a == b,
            "!=" => a != b,
            _ => panic!("invalid operator character found in boolean expression")
        }
    }
}
impl Operation for Assignment {
    fn exec (&self, env: &mut Environment) {
        let val = self.expr.eval();
        println!("setting '{}' to {}", self.ident, val);
        env.vars.insert(self.ident.clone(), val);
    }
}

impl Operation for If {
    fn exec (&self, env: &mut Environment) {
        if self.bexpr.eval() {
            println!("conditional boolean expression evaluated to true, running block");
            self.block.exec(env);
        }
    }
}
