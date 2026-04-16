mod lexer;
mod parser;

use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("+-");

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == kiln_core::token::Token::EOF {
            break;
        }
    }
}
