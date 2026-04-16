#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
}
