use cranelift::prelude::*;

use super::super::function_translator::FunctionTranslator;

impl <'a>FunctionTranslator<'a> {
    pub fn number_gen(&mut self, num: i32) -> Value {
        self.builder.ins().iconst(self.int, i64::from(num))
    }
}