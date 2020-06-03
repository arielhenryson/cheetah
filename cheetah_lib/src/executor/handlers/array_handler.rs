use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::values::Value;
use crate::types::expression::Expression;

impl Executor {
    pub fn array_handler(&mut self, items: &Vec<Expression>) -> ExecutorResults {
        let mut array: Vec<Value> = Vec::new();

        for item in items.iter() {
            let res = self.run(
                item.clone()
            )?;

            array.push(res)
        }

        Ok(
            Value::Array(array)
        )
    }
}