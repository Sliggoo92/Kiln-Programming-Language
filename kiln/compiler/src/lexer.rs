pub use crate::token::Token;

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.get(self.pos).copied();
        self.pos += 1;
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        // consume everything after //
        while let Some(c) = self.advance() {
            if c == '\n' {
                break;
            }
        }
    }
//for keyword matching with tokens from token.rs
    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "main"     => Token::Main,
            "func"     => Token::Func,
            "let"      => Token::Let,
            "const"    => Token::Const,
            "export"   => Token::Export,
            "use"      => Token::Use,
            "struct"   => Token::Struct,
            "return"   => Token::Return,
            "if"       => Token::If,
            "else"     => Token::Else,
            "then"     => Token::Then,
            "end"      => Token::End,
            "while"    => Token::While,
            "for"      => Token::For,
            "loop"     => Token::Loop,
            "break"    => Token::Break,
            "continue" => Token::Continue,
            "and"      => Token::And,
            "or"       => Token::Or,
            "not"      => Token::Not,
            "true"     => Token::Bool(true),
            "false"    => Token::Bool(false),
            "int"      => Token::TypeInt,
            "float"    => Token::TypeFloat,
            "bool"     => Token::TypeBool,
            "string"   => Token::TypeString,
            "byte"     => Token::TypeByte,
            "ptr"      => Token::TypePtr,
            _          => Token::Identifier(ident),
        }
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();
        let mut is_float = false;

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                num.push(c);
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::Float(num.parse().unwrap())
        } else {
            Token::Int(num.parse().unwrap())
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // eat opening "
        let mut s = String::new();
        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }
            s.push(c);
        }
        Token::StringLit(s)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current() {
            None => Token::Eof,

            Some('/') if self.peek() == Some('/') => {
                self.skip_line_comment();
                self.next_token() // recurse to get next real token
            }

            Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(),
            Some(c) if c.is_ascii_digit() => self.read_number(),
            Some('"') => self.read_string(),

            Some('+') => {
                self.advance();
                match self.current() {
                    Some('+') => { self.advance(); Token::PlusPlus }
                    Some('=') => { self.advance(); Token::PlusAssign }
                    _ => Token::Plus,
                }
            }
            Some('-') => {
                self.advance();
                match self.current() {
                    Some('-') => { self.advance(); Token::MinusMinus }
                    Some('=') => { self.advance(); Token::MinusAssign }
                    _ => Token::Minus,
                }
            }
            Some('*') => {
                self.advance();
                if self.current() == Some('=') { self.advance(); Token::StarAssign }
                else { Token::Star }
            }
            Some('/') => {
                self.advance();
                if self.current() == Some('=') { self.advance(); Token::SlashAssign }
                else { Token::Slash }
            }
            Some('%') => { self.advance(); Token::Percent }
            Some('=') => {
                self.advance();
                if self.current() == Some('=') { self.advance(); Token::Eq }
                else { Token::Assign }
            }
            Some('!') => {
                self.advance();
                if self.current() == Some('=') { self.advance(); Token::NotEq }
                else { panic!("unexpected character '!'") }
            }
            Some('<') => {
                self.advance();
                if self.current() == Some('=') { self.advance(); Token::LtEq }
                else { Token::Lt }
            }
            Some('>') => {
                self.advance();
                if self.current() == Some('=') { self.advance(); Token::GtEq }
                else { Token::Gt }
            }
            Some(':') => { self.advance(); Token::Colon }
            Some(';') => { self.advance(); Token::Semicolon }
            Some(',') => { self.advance(); Token::Comma }
            Some('.') => { self.advance(); Token::Dot }
            Some('(') => { self.advance(); Token::LParen }
            Some(')') => { self.advance(); Token::RParen }
            Some('[') => { self.advance(); Token::LBracket }
            Some(']') => { self.advance(); Token::RBracket }

            Some(c) => panic!("unexpected character: '{}'", c),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let done = tok == Token::Eof;
            tokens.push(tok);
            if done { break; }
        }
        tokens
    }
}

