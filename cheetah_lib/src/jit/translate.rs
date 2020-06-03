use cranelift::prelude::*;

use super::jit::JIT;
use super::function_translator::FunctionTranslator;
use crate::types::expression::Expression;

impl JIT {
    pub fn translate(&mut self, expression: Expression) -> Result<(), String> {
        // Our toy language currently only supports I32 values, though Cranelift
        // supports other types.
        let int = self.module.target_config().pointer_type();

        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        self.ctx.func.signature.returns.push(
            AbiParam::new(int)
        );

        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Create the entry block, to start emitting code in.
        let entry_ebb = builder.create_ebb();


        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        builder.append_ebb_params_for_function_params(entry_ebb);


        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_ebb);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_ebb);

        let mut trans = FunctionTranslator {
            int,
            builder,
            module: &mut self.module,
            is_return: false
        };

        let val = trans.translate_expr(expression);
        // let return_variable = trans.variables.get(&the_return).unwrap();
        // let return_value = trans.builder.use_var(val.clone());

        trans.builder.ins().return_(&[val]);

        trans.builder.finalize();

        Ok(())
    }
}
