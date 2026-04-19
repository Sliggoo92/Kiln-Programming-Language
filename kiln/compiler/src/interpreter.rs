use crate::ast::{Stmt, Expr};

pub struct Interpreter;

impl Interpreter {
    /// Temporary debug runner for AST output.
    /// This is NOT a real runtime — only for verifying parsing correctness.
    pub fn run(statements: Vec<Stmt>) {
        for stmt in statements {
            Self::execute(stmt);
        }
    }

    fn execute(stmt: Stmt) {
        match stmt {
            Stmt::ExprStmt(expr) => {
                println!("{}", Self::eval(&expr));
            }
        }
    }
    fn eval(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::Identifier(name) => name.clone(),

            Expr::Binary { left, operator, right } => {
                let l = Self::eval(left);
                let r = Self::eval(right);
                format!("({} {} {})", l, operator, r)
            }
        }
    }
}