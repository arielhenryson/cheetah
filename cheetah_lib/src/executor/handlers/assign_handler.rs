use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::expression::{ Expression, ExpressionDef };

impl Executor {
    pub fn assign_handler(&mut self, ref_expr: &Expression, expr: &Expression) -> ExecutorResults {
        let val = self.run(expr.clone())?;

        match ref_expr.def {
            ExpressionDef::Identifier(ref identifier, ref _debug_info) => {
                self.env.update(
                    identifier,
                    Box::new(val.clone())
                )
            }
            _ => ()
        }

        Ok(val)
    }
}