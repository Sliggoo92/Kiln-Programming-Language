#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    StringType,
    Byte,
    Ptr,
    Array(Box<Type>, usize),
    Array2D(Box<Type>, usize, usize),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, NotEq, Lt, Gt, LtEq, GtEq,
    And, Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
}


#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StringLit(String),
    Identifier(String),
    BinaryOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Option<Expr>,
    },
    Const {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
    Assign {
        target: String,
        value: Expr,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    ExprStmt(Expr),
    If {
        condition: Expr,
        body: Vec<Stmt>,
        else_ifs: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Box<Stmt>,
        condition: Expr,
        step: Box<Stmt>,
        body: Vec<Stmt>,
    },
    Loop {
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct FuncDef {
    pub exported: bool,
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub exported: bool,
    pub name: String,
    pub fields: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct UseDecl {
    pub path: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Use(UseDecl),
    Func(FuncDef),
    Struct(StructDef),
    Main(Vec<Stmt>),
    Let {
        exported: bool,
        name: String,
        ty: Option<Type>,
        value: Option<Expr>,
    },
    Const {
        exported: bool,
        name: String,
        ty: Option<Type>,
        value: Expr,
    },
}