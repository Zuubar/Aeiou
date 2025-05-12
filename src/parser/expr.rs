use crate::lexer::Token;
use crate::parser::types::Type;

#[derive(Debug)]
pub enum Expr {
    Binary(Type, Box<Expr>, Token, Box<Expr>),
    Grouping(Type, Box<Expr>),
    Unary(Type, Box<Expr>),
    Literal(Type, String),
}
