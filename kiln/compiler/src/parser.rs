use crate::lexer::Token;
use crate::ast::{Expr, Stmt, TopLevel, FuncDef, Param, Type, BinOp, UseDecl};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.current() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("expected {:?} but got {:?}", expected, self.current()))
        }
    }

    // skip optional semicolons between statements
    fn skip_semicolons(&mut self) {
        while self.current() == &Token::Semicolon {
            self.advance();
        }
    }

    // --- Top Level ---

    pub fn parse_program(&mut self) -> Result<Vec<TopLevel>, String> {
        let mut items = Vec::new();
        while self.current() != &Token::Eof {
            self.skip_semicolons();
            if self.current() == &Token::Eof {
                break;
            }
            items.push(self.parse_top_level()?);
        }
        Ok(items)
    }

    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        match self.current().clone() {
            Token::Use => self.parse_use(),
            Token::Export => self.parse_export(),
            Token::Func => Ok(TopLevel::Func(self.parse_func(false)?)),
            Token::Let => self.parse_top_let(false),
            Token::Const => self.parse_top_const(false),
            other => Err(format!("unexpected top-level token: {:?}", other)),
        }
    }

    // --- use console; or use console.print; ---
    fn parse_use(&mut self) -> Result<TopLevel, String> {
        self.advance(); // eat 'use'
        let mut path = Vec::new();

        match self.current().clone() {
            Token::Identifier(name) => {
                path.push(name);
                self.advance();
            }
            other => return Err(format!("expected module name, got {:?}", other)),
        }

        // handle use console.print style
        while self.current() == &Token::Dot {
            self.advance(); // eat '.'
            match self.current().clone() {
                Token::Identifier(name) => {
                    path.push(name);
                    self.advance();
                }
                other => return Err(format!("expected identifier after '.', got {:?}", other)),
            }
        }

        self.skip_semicolons();
        Ok(TopLevel::Use(UseDecl { path }))
    }

    // --- export func / export let / export const ---
    fn parse_export(&mut self) -> Result<TopLevel, String> {
        self.advance(); // eat 'export'
        match self.current().clone() {
            Token::Func => Ok(TopLevel::Func(self.parse_func(true)?)),
            Token::Let => self.parse_top_let(true),
            Token::Const => self.parse_top_const(true),
            other => Err(format!("expected func/let/const after export, got {:?}", other)),
        }
    }

    // --- func name(params): return_type then ... end ---
    fn parse_func(&mut self, exported: bool) -> Result<FuncDef, String> {
        self.advance(); // eat 'func'

        let name = match self.current().clone() {
            Token::Identifier(n) => { self.advance(); n }
            other => return Err(format!("expected function name, got {:?}", other)),
        };

        self.expect(&Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(&Token::RParen)?;

        // optional return type
        let return_type = if self.current() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(&Token::Then)?;
        let body = self.parse_block()?;
        self.expect(&Token::End)?;

        Ok(FuncDef { exported, name, params, return_type, body })
    }

    // parse comma-separated parameters: name: type, name: type
    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        while self.current() != &Token::RParen {
            let name = match self.current().clone() {
                Token::Identifier(n) => { self.advance(); n }
                other => return Err(format!("expected param name, got {:?}", other)),
            };
            self.expect(&Token::Colon)?;
            let ty = self.parse_type()?;
            params.push(Param { name, ty });

            if self.current() == &Token::Comma {
                self.advance();
            }
        }
        Ok(params)
    }

    // parse a type annotation: int, float, bool, string, byte, ptr, int[10], int[10][10]
    fn parse_type(&mut self) -> Result<Type, String> {
        let base = match self.current().clone() {
            Token::TypeInt    => { self.advance(); Type::Int }
            Token::TypeFloat  => { self.advance(); Type::Float }
            Token::TypeBool   => { self.advance(); Type::Bool }
            Token::TypeString => { self.advance(); Type::StringType }
            Token::TypeByte   => { self.advance(); Type::Byte }
            Token::TypePtr    => { self.advance(); Type::Ptr }
            other => return Err(format!("expected type, got {:?}", other)),
        };

        // check for array dimensions: int[10] or int[10][10]
        if self.current() == &Token::LBracket {
            self.advance();
            let size = match self.current().clone() {
                Token::Int(n) => { self.advance(); n as usize }
                other => return Err(format!("expected array size, got {:?}", other)),
            };
            self.expect(&Token::RBracket)?;

            // check for second dimension
            if self.current() == &Token::LBracket {
                self.advance();
                let size2 = match self.current().clone() {
                    Token::Int(n) => { self.advance(); n as usize }
                    other => return Err(format!("expected array size, got {:?}", other)),
                };
                self.expect(&Token::RBracket)?;
                return Ok(Type::Array2D(Box::new(base), size, size2));
            }

            return Ok(Type::Array(Box::new(base), size));
        }

        Ok(base)
    }

    // --- parse a block of statements until 'end', 'else', or 'else if' ---
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        loop {
            self.skip_semicolons();
            match self.current() {
                Token::End | Token::Else | Token::Eof => break,
                _ => stmts.push(self.parse_stmt()?),
            }
        }
        Ok(stmts)
    }

    // --- Statements ---
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        let stmt = match self.current().clone() {
            Token::Let      => self.parse_let(),
            Token::Const    => self.parse_const(),
            Token::Return   => self.parse_return(),
            Token::Break    => { self.advance(); self.skip_semicolons(); Ok(Stmt::Break) }
            Token::Continue => { self.advance(); self.skip_semicolons(); Ok(Stmt::Continue) }
            Token::If       => self.parse_if(),
            Token::While    => self.parse_while(),
            Token::For      => self.parse_for(),
            Token::Loop     => self.parse_loop(),
            _               => self.parse_expr_or_assign(),
        }?;
        self.skip_semicolons();
        Ok(stmt)
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // eat 'let'
        let name = match self.current().clone() {
            Token::Identifier(n) => { self.advance(); n }
            other => return Err(format!("expected variable name, got {:?}", other)),
        };
        let ty = if self.current() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let value = if self.current() == &Token::Assign {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Stmt::Let { name, ty, value })
    }

    fn parse_const(&mut self) -> Result<Stmt, String> {
        self.advance(); // eat 'const'
        let name = match self.current().clone() {
            Token::Identifier(n) => { self.advance(); n }
            other => return Err(format!("expected name, got {:?}", other)),
        };
        let ty = if self.current() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(&Token::Assign)?;
        let value = self.parse_expr()?;
        Ok(Stmt::Const { name, ty, value })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance(); // eat 'return'
        if matches!(self.current(), Token::Semicolon | Token::End) {
            Ok(Stmt::Return(None))
        } else {
            Ok(Stmt::Return(Some(self.parse_expr()?)))
        }
    }

    // if condition then ... else if condition then ... else then ... end
    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // eat 'if'
        let condition = self.parse_expr()?;
        self.expect(&Token::Then)?;
        let body = self.parse_block()?;

        let mut else_ifs = Vec::new();
        let mut else_body = None;

        while self.current() == &Token::Else {
            self.advance(); // eat 'else'
            if self.current() == &Token::If {
                self.advance(); // eat 'if'
                let ei_cond = self.parse_expr()?;
                self.expect(&Token::Then)?;
                let ei_body = self.parse_block()?;
                else_ifs.push((ei_cond, ei_body));
            } else {
                // plain else — may optionally have 'then'
                if self.current() == &Token::Then {
                    self.advance();
                }
                else_body = Some(self.parse_block()?);
                break;
            }
        }

        self.expect(&Token::End)?;
        Ok(Stmt::If { condition, body, else_ifs, else_body })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // eat 'while'
        let condition = self.parse_expr()?;
        self.expect(&Token::Then)?;
        let body = self.parse_block()?;
        self.expect(&Token::End)?;
        Ok(Stmt::While { condition, body })
    }

    // for i: int = 0; i < 10; i++ then ... end
    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance(); // eat 'for'
        let init = Box::new(self.parse_let()?);
        self.expect(&Token::Semicolon)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::Semicolon)?;
        let step = Box::new(self.parse_expr_or_assign()?);
        self.expect(&Token::Then)?;
        let body = self.parse_block()?;
        self.expect(&Token::End)?;
        Ok(Stmt::For { init, condition, step, body })
    }

    fn parse_loop(&mut self) -> Result<Stmt, String> {
        self.advance(); // eat 'loop'
        self.expect(&Token::Then)?;
        let body = self.parse_block()?;
        self.expect(&Token::End)?;
        Ok(Stmt::Loop { body })
    }

    // either a bare expression or an assignment: x = expr
    fn parse_expr_or_assign(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;

        if self.current() == &Token::Assign {
            self.advance();
            let value = self.parse_expr()?;
            if let Expr::Identifier(name) = expr {
                return Ok(Stmt::Assign { target: name, value });
            } else {
                return Err("left side of assignment must be a variable name".to_string());
            }
        }

        Ok(Stmt::ExprStmt(expr))
    }

    // top-level let/const with optional export flag
    fn parse_top_let(&mut self, exported: bool) -> Result<TopLevel, String> {
        self.advance(); // eat 'let'
        let name = match self.current().clone() {
            Token::Identifier(n) => { self.advance(); n }
            other => return Err(format!("expected name, got {:?}", other)),
        };
        let ty = if self.current() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let value = if self.current() == &Token::Assign {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.skip_semicolons();
        Ok(TopLevel::Let { exported, name, ty, value })
    }

    fn parse_top_const(&mut self, exported: bool) -> Result<TopLevel, String> {
        self.advance(); // eat 'const'
        let name = match self.current().clone() {
            Token::Identifier(n) => { self.advance(); n }
            other => return Err(format!("expected name, got {:?}", other)),
        };
        let ty = if self.current() == &Token::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(&Token::Assign)?;
        let value = self.parse_expr()?;
        self.skip_semicolons();
        Ok(TopLevel::Const { exported, name, ty, value })
    }

    // --- Expressions ---

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_unary()?;
        self.parse_binop(0, lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.current() == &Token::Not {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: crate::ast::UnaryOp::Not,
                expr: Box::new(expr),
            });
        }
        if self.current() == &Token::Minus {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: crate::ast::UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }
        self.parse_primary()
    }

    fn token_precedence(tok: &Token) -> i32 {
        match tok {
            Token::Or                          => 1,
            Token::And                         => 2,
            Token::Eq | Token::NotEq           => 3,
            Token::Lt | Token::Gt |
            Token::LtEq | Token::GtEq          => 4,
            Token::Plus | Token::Minus         => 5,
            Token::Star | Token::Slash |
            Token::Percent                     => 6,
            _                                  => -1,
        }
    }

    fn token_to_binop(tok: &Token) -> Option<BinOp> {
        match tok {
            Token::Plus    => Some(BinOp::Add),
            Token::Minus   => Some(BinOp::Sub),
            Token::Star    => Some(BinOp::Mul),
            Token::Slash   => Some(BinOp::Div),
            Token::Percent => Some(BinOp::Mod),
            Token::Eq      => Some(BinOp::Eq),
            Token::NotEq   => Some(BinOp::NotEq),
            Token::Lt      => Some(BinOp::Lt),
            Token::Gt      => Some(BinOp::Gt),
            Token::LtEq    => Some(BinOp::LtEq),
            Token::GtEq    => Some(BinOp::GtEq),
            Token::And     => Some(BinOp::And),
            Token::Or      => Some(BinOp::Or),
            _              => None,
        }
    }

    fn parse_binop(&mut self, min_prec: i32, mut lhs: Expr) -> Result<Expr, String> {
        loop {
            let prec = Self::token_precedence(self.current());
            if prec < min_prec {
                break;
            }
            let op_tok = self.current().clone();
            let op = Self::token_to_binop(&op_tok).unwrap();
            self.advance();
            let mut rhs = self.parse_unary()?;
            let next_prec = Self::token_precedence(self.current());
            if next_prec > prec {
                rhs = self.parse_binop(prec + 1, rhs)?;
            }
            lhs = Expr::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current().clone() {
            Token::Int(n) => {
                self.advance();
                Ok(Expr::IntLit(n))
            }
            Token::Float(f) => {
                self.advance();
                Ok(Expr::FloatLit(f))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Expr::BoolLit(b))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s))
            }
            Token::Identifier(name) => {
                self.advance();
                // function call?
                if self.current() == &Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current() != &Token::RParen {
                        args.push(self.parse_expr()?);
                        if self.current() == &Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(&Token::RParen)?;
                    return Ok(Expr::Call { callee: name, args });
                }
                // field access: module.symbol
                if self.current() == &Token::Dot {
                    self.advance();
                    let field = match self.current().clone() {
                        Token::Identifier(f) => { self.advance(); f }
                        other => return Err(format!("expected field name, got {:?}", other)),
                    };
                    // could be a method call too: obj.method(args)
                    if self.current() == &Token::LParen {
                        self.advance();
                        let mut args = Vec::new();
                        while self.current() != &Token::RParen {
                            args.push(self.parse_expr()?);
                            if self.current() == &Token::Comma {
                                self.advance();
                            }
                        }
                        self.expect(&Token::RParen)?;
                        return Ok(Expr::Call {
                            callee: format!("{}.{}", name, field),
                            args,
                        });
                    }
                    return Ok(Expr::FieldAccess {
                        object: Box::new(Expr::Identifier(name)),
                        field,
                    });
                }
                Ok(Expr::Identifier(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            other => Err(format!("unexpected token in expression: {:?}", other)),
        }
    }
                 }
