use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}


