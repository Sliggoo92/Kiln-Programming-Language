#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Const,
    Func,
    If,
    Else,
    Then,
    End,
    While,
    For,
    Loop,
    Break,
    Continue,
    Return,
    Use,
    Export,

    // Identifiers + literals
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Assign,      // =
    Equal,       // ==
    NotEqual,    // !=
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    Colon,
    Semicolon,
    Comma,
    Dot,

    LParen,
    RParen,
    LBracket,
    RBracket,

    EOF,
}
