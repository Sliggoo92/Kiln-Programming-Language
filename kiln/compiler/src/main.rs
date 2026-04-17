mod lexer;
mod parser;
mod ast;
mod compiler;

use ast::TopLevel;
use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let context = inkwell::context::Context::create();
    let mut compiler = Compiler::new(&context, "my_program");

    // source code of your program
    let source = std::fs::read_to_string("program.kiln").expect("could not read file");
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().expect("parse error");

    for item in &ast {
        match item {
            TopLevel::Func(func_def) => {
                compiler.codegen_func(func_def).expect("codegen error");
            }
            TopLevel::Use(_) => {
                // module resolution handled separately
            }
            _ => {}
        }
    }

    // emit LLVM IR to stdout for now
    compiler.module.print_to_stderr();

}
