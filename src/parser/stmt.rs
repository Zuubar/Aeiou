use crate::parser::{Expr, Type};

pub enum Stmt {
    Print(Type, Box<Expr>),
    Expression(Type, Box<Expr>),
}
