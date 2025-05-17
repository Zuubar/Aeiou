use crate::parser::{Expr, Type};

pub enum Stmt {
    Var(Type, String, Box<Expr>),
    Print(Box<Expr>),
    Expression(Box<Expr>),
}
