use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::values::Value;

impl Executor {
    pub fn number_handler(&mut self, num: i32) -> ExecutorResults {
        Ok(
            Value::Integer(num)
        )
    }
}