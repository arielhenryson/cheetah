use cranelift::prelude::*;

use super::super::function_translator::FunctionTranslator;
use crate::types::expression::Expression;

impl <'a>FunctionTranslator<'a> {
    pub fn binary_expression_gen(&mut self, a: &Box<Expression>,
                                     _op: &String, b: &Box<Expression>) -> Value {
        let a_res = self.translate_expr(*a.clone());
        let b_res = self.translate_expr(*b.clone());

        self.builder.ins().iadd(a_res, b_res)
    }
}