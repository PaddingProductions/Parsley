use crate::ast::Types;
use std::collections::{HashMap, HashSet};

pub struct Environment {
    vars: HashMap<String, Vec<Types>>,
    scopes: Vec<HashSet<String>>
}

impl Environment {
    pub fn new () -> Self {
        Self { 
            vars:   HashMap::new(),
            scopes: vec![HashSet::new()]
        }
    }

    // Sets Variable
    pub fn set(&mut self, ident: &String, val: Types) {

        /* Check if top scope includes identifier. 
         * -> Identifies if a new variable is declared or a value is re-set
         */ 
        let scope = self.scopes.last_mut().expect("Unexpected: Scope stack empty");
        let new = !scope.contains(ident);

        // If is new variable, insert into scope and append to identifier stack
        if new {
            scope.insert(ident.clone());
            if let Some(v) = self.vars.get_mut(ident) { 
                v.push(val);
            } else {
                self.vars.insert(ident.clone(), vec![val]);
            }

        // If overwritting, set last on identifier stack.
        } else {
             let v = self.vars.get_mut(ident).expect("Unexpected: Scope set includes identifier, but identifier not found in vars map");
             let last = v.last_mut().expect("Unexpected: Vars map includes identifier, but mapped stack empty");
             *last = val;
        }

    }

    #[cfg(test)]
    // Convienece function for testing
    pub fn test(&self, ident: &str) -> &Types {
        self.var(&ident.to_owned()).unwrap()
    }


    // Gets variable
    pub fn var(&self, ident: &String) -> Result<&Types, InterpreterErr> {
        if let Some(stack) = self.vars.get(ident) {
            if let Some (val) = stack.last() {
                Ok(val)
            } else {
                inter_err("Variable {ident} is not defined")
            }
        } else {
            inter_err("Variable {ident} is not defined")
        }
    }

    // Introduce new scope
    pub fn new_scope (&mut self) {
        self.scopes.push(HashSet::new());
    }

    // Delete scope
    pub fn exit_scope (&mut self) {
        if self.scopes.len() == 1 {
            panic!("Unexpected: Attempted to exit global scope");
        }

        // Get scope
        let scope = self.scopes.pop().expect("Unexpected: Scope stack empty on");

        // Pop identifiers from variable stack
        for ident in scope {
            let stack = self.vars.get_mut(&ident).expect("Unexpected: Identifier included in scope but not in vars map");
            stack.pop();
        }
    }
}

#[derive(Debug)]
pub enum InterpreterErr {
    Err(String),
}
pub fn inter_err<T> (s: &str) -> Result<T, InterpreterErr> {
    Err(InterpreterErr::Err(s.to_owned()))
}
