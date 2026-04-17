use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::OptimizationLevel;

pub struct KilnJIT<'ctx> {
    pub engine: ExecutionEngine<'ctx>,
}

impl<'ctx> KilnJIT<'ctx> {
    pub fn new(module: &'ctx Module<'ctx>) -> Result<Self, String> {
        let engine = module
            .create_jit_execution_engine(OptimizationLevel::Default)
            .map_err(|e| e.to_string())?;

        Ok(KilnJIT { engine })
    }

    // run a function that takes no arguments and returns i64
    // used for top-level expressions and the main block
    pub unsafe fn run_function(&self, name: &str) -> i64 {
        let func = self.engine
            .get_function::<unsafe extern "C" fn() -> i64>(name)
            .expect("function not found in JIT");
        func.call()
    }
}
