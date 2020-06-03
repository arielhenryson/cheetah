use std::collections::HashMap;

use crate::types::values::HeapValue;

#[derive(Debug, Clone)]
pub struct Scope {
    pub records: HashMap<String, HeapValue>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            records: HashMap::new()
        }
    }

    pub fn push(&mut self, name: &String, value: HeapValue) {
        self.records.insert(
            name.clone(),
            value
        );
    }

    pub fn update(&mut self, name: &String, value: HeapValue) {
        match self.records.get(name) {
            Some(_v) => {},
            None => {
                eprintln!("Error `{}` is undefined", name);
                std::process::exit(1);
            }
        }

        self.records.remove(name);
        self.records.insert(
            name.clone(),
            value
        );
    }

    pub fn get(&mut self, name: &String) -> Result<HeapValue, ()> {
        match self.records.get(name) {
            Some(v) => {
                Ok(v.clone())
            },
            None => {
                Err(())
            }
        }
    }
}