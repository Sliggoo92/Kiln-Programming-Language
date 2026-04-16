mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parserl

fn main() {
    let source = "42";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    
    println!("Parser initialized!");
    
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == kiln_core::token::Token::EOF {
            break;
        }
    }
}
