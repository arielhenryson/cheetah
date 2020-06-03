use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::values::Value;
use crate::types::expression::HeapExpr;

impl Executor {
    pub fn variable_handler(&mut self, name: &String, expr: &HeapExpr) -> ExecutorResults {
        let value = self.run(*expr.clone())?;

        self.env.push(
            name,
            Box::new(value)
        );

        Ok(
            Value::Undefined
        )
    }
}