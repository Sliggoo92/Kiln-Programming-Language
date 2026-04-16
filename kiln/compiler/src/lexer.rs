use kiln_core::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
        }
    }
    fn current(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        match self.current() {
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            None => Token::EOF,
            _ => {
                self.advance();
                self.next_token()
            }
        }
    }
}

