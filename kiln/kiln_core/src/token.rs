#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    StringLit(String),
    Bool(bool),

    // Identifier
    Identifier(String),

    // Keywords
    Func,
    Let,
    Const,
    Export,
    Use,
    Struct,
    Return,
    If,
    Else,
    Then,
    End,
    While,
    For,
    Loop,
    Break,
    Continue,
    And,
    Or,
    Not,

    // Types
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeString,
    TypeByte,
    TypePtr,

    // Symbols
    Colon,        // :
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    LParen,       // (
    RParen,       // )
    LBracket,     // [
    RBracket,     // ]

    // Arithmetic
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    PlusPlus,     // ++
    MinusMinus,   // --

    // Assignment
    Assign,       // =
    PlusAssign,   // +=
    MinusAssign,  // -=
    StarAssign,   // *=
    SlashAssign,  // /=

    // Comparison
    Eq,           // ==
    NotEq,        // !=
    Lt,           // <
    Gt,           // >
    LtEq,         // <=
    GtEq,         // >=

    // Special
    Eof,
}    Less,
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
