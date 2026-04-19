use inkwell::context::Context;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: inkwell::module::Module<'ctx>,
    pub builder: inkwell::builder::Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("kiln");
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
        }
    }
}