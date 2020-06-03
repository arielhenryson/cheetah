use cranelift::prelude::*;

use super::super::function_translator::FunctionTranslator;
use crate::types::expression::{ Expression };

impl <'a>FunctionTranslator<'a> {
    pub fn block_gen(&mut self, expressions: Vec<Expression>) -> Value {
        for n in expressions.iter() {
            // execute expression inside the block
            let res = self.translate_expr(
                n.clone()
            );

            // after executing each expression inside the block
            // we need to make sure that if the expression return something
            // we stop the block from executing the remaining expressions
            // and return the value of the last expression
            if self.is_return {
                return res;
            }
        }

        self.builder.ins().iconst(self.int, -1)
    }
}