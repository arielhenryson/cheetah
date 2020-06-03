use cranelift::prelude::*;

use super::super::function_translator::FunctionTranslator;
use crate::types::expression::Expression;

impl <'a>FunctionTranslator<'a> {
    pub fn return_gen(&mut self, return_value: &Option<Box<Expression>>) -> Value {
        let result = match *return_value {
            Some(ref expr) => self.translate_expr(*expr.clone()),
            None => self.builder.ins().iconst(self.int, i64::from(-1)),
        };

        // Set flag for return
        self.is_return = true;

        result
    }
}