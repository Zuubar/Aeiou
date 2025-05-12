mod expr;
mod stmt;
mod types;

use crate::lexer::Token;
use crate::lexer::Token::Newline;
pub use crate::parser::expr::Expr;
pub use crate::parser::stmt::Stmt;
pub use crate::parser::types::Type;
use std::iter::Peekable;
use std::slice::Iter;

fn match_token(iter: &mut Peekable<Iter<Token>>, token: Token) -> Result<(), &'static str> {
    if iter.peek().is_some() && **iter.peek().unwrap() != token {
        return Err("Expected a newline.");
    }
    Ok(())
}

fn primary(iter: &mut Peekable<Iter<Token>>) -> Result<(Type, Box<Expr>), &'static str> {
    match iter.next() {
        Some(Token::LeftParen) => {
            let (t, expr) = expression(iter)?;

            match iter.next() {
                Some(Token::RightParen) => Ok((t, Box::new(Expr::Grouping(t, expr)))),
                _ => Err("Expected closing ')'."),
            }
        }
        Some(Token::Minus) => {
            let (t, expr) = primary(iter)?;
            Ok((t, Box::new(Expr::Unary(t, expr))))
        }
        Some(Token::Number(t, literal_str)) => {
            let mut literal = literal_str.clone();
            if literal_str.ends_with('.') {
                literal.remove(literal.len() - 1);
            }
            Ok((*t, Box::new(Expr::Literal(*t, literal))))
        }
        _ => Err("Expected an expression."),
    }
}

fn factor(iter: &mut Peekable<Iter<Token>>) -> Result<(Type, Box<Expr>), &'static str> {
    let (left_t, mut left_expr) = primary(iter)?;

    while let Some(Token::Star | Token::Slash) = iter.peek() {
        let operator = iter.next().unwrap();
        let (right_t, right_expr) = primary(iter)?;
        if left_t != right_t {
            return Err("Type mismatch.");
        }
        left_expr = Box::new(Expr::Binary(
            left_t,
            left_expr,
            operator.clone(),
            right_expr,
        ));
    }

    Ok((left_t, left_expr))
}

fn term(iter: &mut Peekable<Iter<Token>>) -> Result<(Type, Box<Expr>), &'static str> {
    let (left_t, mut left_expr) = factor(iter)?;

    while let Some(Token::Plus | Token::Minus) = iter.peek() {
        let operator = iter.next().unwrap();
        let (right_t, right_expr) = factor(iter)?;
        if left_t != right_t {
            return Err("Type mismatch.");
        }
        left_expr = Box::new(Expr::Binary(
            left_t,
            left_expr,
            operator.clone(),
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
        match iter.peek().unwrap() {
            Token::Print => {
                iter.next();
                let (t, expr) = expression(iter)?;
                match_token(iter, Newline)?;
                statements.push(Stmt::Print(t, expr));
                iter.next();
            }
            _ => {
                let (t, expr) = expression(iter)?;
                match_token(iter, Newline)?;
                statements.push(Stmt::Expression(t, expr));
                iter.next();
            }
        }
    }

    Ok(statements)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, &'static str> {
    let mut iter = tokens.iter().peekable();
    declaration(&mut iter)
}
