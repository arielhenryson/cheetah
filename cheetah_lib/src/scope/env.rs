use std::collections::HashMap;

use super::scope::Scope;
use super::scope_path::get_above_scope_path;
use crate::types::values::HeapValue;

// represent the scope tree of the running
// application
#[derive(Debug)]
pub struct Env {
    active_scope_path: String,
    scopes: HashMap<String, Scope>
}

impl Env {
    pub fn new() -> Self {
        let mut scopes = HashMap::new();
        let scope = Scope::new();
        let active_scope_path = String::from("");

        scopes.insert(
            active_scope_path.clone(),
            scope
        );

        Self {
            active_scope_path,
            scopes
        }
    }

    // get scope will try to find the scope by the path you pass to it
    // if you dont pass a path it will return the current scope
    pub fn get_scope(&mut self, path: Option<String>) -> Scope {
        match path {
            Some(val) => {
                self.scopes.get(&val).unwrap().clone()
            },
            None => {
                self.scopes.get(&self.active_scope_path).unwrap().clone()
            }
        }
    }

    // push will push a new record to the current scope
    pub fn push(&mut self, name: &String, value: HeapValue) {
        let mut scope = self.get_scope(None);
        scope.push(name, value);

        self.scopes.remove(&self.active_scope_path);
        self.scopes.insert(
            self.active_scope_path.clone(),
            scope
        );
    }

    // update will update a record in the current scope
    pub fn update(&mut self, name: &String, value: HeapValue) {
        let mut scope = self.get_scope(None);

        println!("{:?}", scope.records);
        println!("{:?}", self.active_scope_path.clone());
        scope.update(name, value)
    }


    // move to the above scope
    pub fn pop(&mut self) {
        let res = get_above_scope_path(self.active_scope_path.clone());
        match res {
            Some(scope) => {
                self.active_scope_path = scope;
            }
            None => {}
        }
    }

    // crate a new empty scope
    pub fn crate_scope(&mut self, name: String) {
        self.active_scope_path = self.active_scope_path.clone() + "." + &name.clone();

        self.scopes.insert(
            self.active_scope_path.clone(),
            Scope::new()
        );
    }

    // will try to get a record from the current scope
    // if can't find it in the current scope will go up in the scope chain
    // and will try to do the same
    pub fn get(&mut self, name: &String) -> Result<HeapValue, ()> {
        let mut current_scope_path = self.active_scope_path.clone();

        loop {
            let mut scope = self.get_scope(
                Some(current_scope_path.clone())
            );

            let res = scope.get(name);

            // if we found a value
            if res.is_ok() {
                return res;
            }

            // If we didn't find the value in the current scope
            // we need to check if there is a scope above
            // and if not return the error
            if current_scope_path == String::from("") {
                return res;
            }

            current_scope_path = get_above_scope_path(
                current_scope_path
            ).unwrap();
        }
    }
}