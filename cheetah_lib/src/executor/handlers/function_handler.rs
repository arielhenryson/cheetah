use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::function::{ Function, RegularFunction };
use crate::types::expression::Expression;
use crate::types::values::{ Value, HeapValue };

impl Executor {
    pub fn function_handler(&mut self, name: &Option<String>,
                                   args: &Vec<Expression>, expr: &Expression) -> ExecutorResults {
        let func = Function::RegularFunc(
            RegularFunction::new(
                name.clone(),
                args.clone(),
                expr.clone()
            )
        );

        let value = Value::Function(
            Box::new(func)
        );

        self.push_to_scope(
            name.clone(),
            Box::new(value.clone())
        );

        Ok(value)
    }

    fn push_to_scope(&mut self, name: Option<String>, value: HeapValue) {
        match name {
            Some(ref n) => {
                self.env.push(n, value)
            },
            None => {}
        };
    }
}