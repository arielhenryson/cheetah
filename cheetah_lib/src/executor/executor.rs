use crate::types::expression::Expression;
use crate::types::values::Value;
use crate::scope::env::Env;
use crate::error_handler::error_handler::ErrorHandler;

pub type ExecutorResults = Result<Value, Value>;

#[derive(Debug)]
pub struct Executor {
    // signal if last expression return something
    pub is_return: bool,

    // represent the scope tree of the running
    // application
    pub env: Env,

    // handle error
    pub error_handler: ErrorHandler
}

impl Executor {
    pub fn new(error_handler: ErrorHandler) -> Self {
        Self {
            is_return: false,
            env: Env::new(),
            error_handler
        }
    }

    pub fn start(&mut self, expression: Expression) -> ExecutorResults {
        self.run(expression)
    }
}