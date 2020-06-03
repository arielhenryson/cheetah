use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::expression::{ Expression, HeapExpr };

impl Executor {
    pub fn if_handler(&mut self, condition: &Expression, expression: &Expression,
                                        else_block: &Option<HeapExpr>) -> ExecutorResults {
        let condition_res = self.run(
            condition.clone()
        )?;

        if condition_res.is_truthy() {
            return Ok(
                self.run(
                    expression.clone()
                )?
            );
        }

        let else_res;

        match else_block {
            Some(else_expr) => {
                else_res = self.run(
                    *else_expr.clone()
                )?;
            },
            None => {
                else_res = self.undefined_handler()?
            }
        }

        Ok(else_res)
    }
}