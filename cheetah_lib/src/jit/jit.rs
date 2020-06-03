use cranelift::prelude::*;
use cranelift_module::{ DataContext, Linkage, Module };
use cranelift_simplejit::{ SimpleJITBackend, SimpleJITBuilder };

use crate::types::expression::Expression;

pub struct JIT {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    pub builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    pub ctx: codegen::Context,

    /// The data context, which is to data objects what `ctx` is to functions.
    pub _data_ctx: DataContext,

    /// The module, with the simplejit backend, which manages the JIT'd
    /// functions.
    pub module: Module<SimpleJITBackend>,
}

impl JIT {
    pub fn new() -> Self {
        let builder = SimpleJITBuilder::new(
            cranelift_module::default_libcall_names()
        );

        let module = Module::new(builder);

        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            _data_ctx: DataContext::new(),
            module,
        }
    }

    pub fn compile(&mut self, expression: Expression) -> Result<*const u8, String> {
        self.translate(expression)?;

        // Next, declare the function to simplejit. Functions must be declared
        // before they can be called, or defined.
        //
        // TODO: This may be an area where the API should be streamlined; should
        // we have a version of `declare_function` that automatically declares
        // the function?
        let id = self
            .module
            .declare_function("func1", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        // Define the function to simplejit. This finishes compilation, although
        // there may be outstanding relocations to perform. Currently, simplejit
        // cannot finish relocations until all functions to be called are
        // defined. For this toy demo for now, we'll just finalize the function
        // below.
        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        // Now that compilation is finished, we can clear out the context state.
        self.module.clear_context(&mut self.ctx);

        // Finalize the functions which we just defined, which resolves any
        // outstanding relocations (patching in addresses, now that they're
        // available).
        self.module.finalize_definitions();

        // We can now retrieve a pointer to the machine code.
        let code = self.module.get_finalized_function(id);

        Ok(code)
    }
}
