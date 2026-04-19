mod token;
mod lexer;
mod parser;
mod ast;
mod compiler;
mod jit;

use ast::TopLevel;
use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;

fn main() {
    inkwell::targets::Target::initialize_native(
        &inkwell::targets::InitializationConfig::default()
    ).expect("failed to initialize native target");

    let context = inkwell::context::Context::create();
    let mut compiler = Compiler::new(&context, "kiln_program");

    let source = std::fs::read_to_string("program.kiln")
        .expect("could not read file");

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().expect("parse error");

    for item in &ast {
        match item {
            TopLevel::Func(func_def) => {
                compiler.codegen_func(&func_def).expect("codegen error");
            }
            TopLevel::Main(body) => {
                let func_def = crate::ast::FuncDef {
                    exported: false,
                    name: "main".to_string(),
                    params: vec![],
                    return_type: None,
                    body: body.clone(),
                };
                compiler.codegen_func(&func_def).expect("codegen error");
            }
            TopLevel::Use(_) => {}
            _ => {}
        }
    }

    compiler.module.print_to_stderr();

    if compiler.module.get_function("main").is_some() {
        let engine = compiler.module
            .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
            .expect("failed to create JIT");
        unsafe {
            let func = engine
                .get_function::<unsafe extern "C" fn() -> i64>("main")
                .expect("main not found in JIT");
            func.call();
        }
    }
}