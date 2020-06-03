use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::expression::Expression;
use crate::types::values::Value;

impl Executor {
    pub fn binary_expression_handler(&mut self, a: &Box<Expression>,
                                     _op: &String, b: &Box<Expression>) -> ExecutorResults {
        let a_res = self.run(*a.clone())?;
        let b_res = self.run(*b.clone())?;

        let mut res = 0;

        match a_res {
            Value::Integer(val) => res += val,
            _ => return Err(Value::Undefined),
        }

        match b_res {
            Value::Integer(val) => res += val,
            _ => return Err(Value::Undefined),
        }

        Ok(
            Value::Integer(res)
        )
    }
}