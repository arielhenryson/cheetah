use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::expression::Expression;
use crate::types::values::Value;

impl Executor {
    pub fn block_handler(&mut self, expressions: Vec<Expression>) -> ExecutorResults {
        for n in expressions.iter() {
            // execute expression inside the block
            let res = self.run(
                n.clone()
            )?;

            // after executing each expression inside the block
            // we need to make sure that if the expression return something
            // we stop the block from executing the remaining expressions
            // and return the value of the last expression
            if self.is_return {
                self.env.pop();
                return Ok(res);
            }
        }

        self.env.pop();
        Ok(Value::Undefined)
    }
}