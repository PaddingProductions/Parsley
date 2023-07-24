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


    // Declare Variable
    pub fn declare(&mut self, ident: &String, val: Types) {

        /* Check if top scope includes identifier. 
         * -> Identifies if this is a valid declaration or a overlap 
         */ 
        let scope = self.scopes.last_mut().expect("Unexpected: Scope stack empty");

        // If is new variable, insert into scope and append to identifier stack
        if !scope.contains(ident) {
            scope.insert(ident.clone());
            if let Some(v) = self.vars.get_mut(ident) { 
                v.push(val);
            } else {
                self.vars.insert(ident.clone(), vec![val]);
            }
        }
    }


    // Sets Variable
    pub fn set(&mut self, ident: &String, val: Types) -> InterpRes<()> {
        if let Some(v) = self.vars.get_mut(ident) {
            let last = v.last_mut().expect("Unexpected: Vars map includes identifier, but mapped stack empty");
            *last = val;
            Ok(())
        } else { 
            Err(InterpreterErr::Err(format!("Variable {ident} is not declared")))
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
                Err(InterpreterErr::Err(format!("Variable {ident} is not defined")))
            }
        } else {
            Err(InterpreterErr::Err(format!("Variable {ident} is not defined")))
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
pub type InterpRes<T> = Result<T, InterpreterErr>;
