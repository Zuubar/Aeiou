mod expr;
mod stmt;
mod types;

use crate::lexer::TokenType::{Equal, Identifier, Newline};
use crate::lexer::{Token, TokenType};
pub use crate::parser::expr::Expr;
pub use crate::parser::stmt::Stmt;
pub use crate::parser::types::Type;
use std::iter::Peekable;
use std::slice::Iter;

fn match_token(iter: &mut Peekable<Iter<Token>>, token_type: TokenType) -> Option<Token> {
    let current = iter.next();
    if current.is_none() || current.unwrap().type_ != token_type {
        return None;
    }
    Some(current.unwrap().clone())
}

fn primary(iter: &mut Peekable<Iter<Token>>) -> Result<(Type, Box<Expr>), &'static str> {
    match iter.next() {
        Some(Token {
            type_: TokenType::LeftParen,
            ..
        }) => {
            let (t, expr) = expression(iter)?;
            match iter.next() {
                Some(Token {
                    type_: TokenType::RightParen,
                    ..
                }) => Ok((t, Box::new(Expr::Grouping(t, expr)))),
                _ => Err("Expected closing ')'."),
            }
        }
        Some(Token {
            type_: TokenType::Minus,
            ..
        }) => {
            let (t, expr) = primary(iter)?;
            Ok((t, Box::new(Expr::Unary(t, expr))))
        }
        Some(
            token @ Token {
                type_: TokenType::Number,
                ..
            },
        ) => {
            let type_ = match token.string.contains(".") {
                true => Type::F64,
                false => Type::I32,
            };
            Ok((type_, Box::new(Expr::Literal(type_, token.string.clone()))))
        }
        _ => Err("Expected an expression."),
    }
}

fn factor(iter: &mut Peekable<Iter<Token>>) -> Result<(Type, Box<Expr>), &'static str> {
    let (left_t, mut left_expr) = primary(iter)?;
    if iter.peek().is_none() {
        return Ok((left_t, left_expr));
    }

    while let TokenType::Star | TokenType::Slash = iter.peek().unwrap().type_ {
        let operator = iter.next().unwrap();
        let (right_t, right_expr) = primary(iter)?;
        if left_t != right_t {
            return Err("Type mismatch.");
        }
        left_expr = Box::new(Expr::Binary(
            left_t,
            left_expr,
            operator.type_.clone(),
            right_expr,
        ));
    }

    Ok((left_t, left_expr))
}

fn term(iter: &mut Peekable<Iter<Token>>) -> Result<(Type, Box<Expr>), &'static str> {
    let (left_t, mut left_expr) = factor(iter)?;
    if iter.peek().is_none() {
        return Ok((left_t, left_expr));
    }

    while let TokenType::Plus | TokenType::Minus = iter.peek().unwrap().type_ {
        let operator = iter.next().unwrap();
        let (right_t, right_expr) = factor(iter)?;
        if left_t != right_t {
            return Err("Type mismatch.");
        }
        left_expr = Box::new(Expr::Binary(
            left_t,
            left_expr,
            operator.type_.clone(),
            right_expr,
        ));
    }

    Ok((left_t, left_expr))
}

fn expression(iter: &mut Peekable<Iter<Token>>) -> Result<(Type, Box<Expr>), &'static str> {
    term(iter)
}

fn declaration(iter: &mut Peekable<Iter<Token>>) -> Result<Vec<Stmt>, &'static str> {
    let mut statements = Vec::new();
    while iter.peek().is_some() {
        match iter.peek().unwrap().type_ {
            TokenType::Print => {
                iter.next();
                let (_t, expr) = expression(iter)?;
                if match_token(iter, Newline).is_none() {
                    return Err("Expected a newline.");
                }
                statements.push(Stmt::Print(expr));
            }
            TokenType::Var => {
                iter.next();
                if match_token(iter, Identifier).is_none() {
                    return Err("Expected an identifier.");
                }

                let name = match_token(iter, Equal);
                if name.is_none() {
                    return Err("Expected an equal operator.");
                }

                let (t, expr) = expression(iter)?;
                if match_token(iter, Newline).is_none() {
                    return Err("Expected a newline.");
                }
                statements.push(Stmt::Var(t, name.unwrap().string, expr));
            }
            _ => {
                let (_t, expr) = expression(iter)?;
                if match_token(iter, Newline).is_none() {
                    return Err("Expected a newline.");
                }
                statements.push(Stmt::Expression(expr));
            }
        }
    }

    Ok(statements)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, &'static str> {
    let mut iter = tokens.iter().peekable();
    declaration(&mut iter)
}
