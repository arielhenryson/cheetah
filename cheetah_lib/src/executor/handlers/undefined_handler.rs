use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::values::Value;

impl Executor {
    pub fn undefined_handler(&mut self) -> ExecutorResults {
        Ok(
            Value::Undefined
        )
    }
}