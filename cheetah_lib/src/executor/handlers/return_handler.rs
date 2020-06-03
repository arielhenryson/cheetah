use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::expression::Expression;
use crate::types::values::Value;

impl Executor {
    pub fn return_handler(&mut self, return_value: &Option<Box<Expression>>) -> ExecutorResults {
        let result = match *return_value {
            Some(ref expr) => self.run(*expr.clone()),
            None => Ok(Value::Undefined),
        };

        // Set flag for return
        self.is_return = true;

        result
    }
}