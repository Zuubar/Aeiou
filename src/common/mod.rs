use crate::lexer::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Expr<'a> {
    Binary(Type, Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Grouping(Type, Box<Expr<'a>>),
    Unary(Type, Box<Expr<'a>>),
    Literal(Type, &'a str),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    I32,
    F64,
}

#[derive(Debug, Clone)]
pub enum Register {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lowered = format!("{:?}", self).to_lowercase();
        write!(f, "{}", lowered)
    }
}
