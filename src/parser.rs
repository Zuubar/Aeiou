use crate::common::{Expr, Type};
use crate::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

fn primary<'a>(iter: &mut Peekable<Iter<'a, Token>>) -> Result<(Box<Expr<'a>>, Type), &'a str> {
    match iter.next() {
        Some(Token::LeftParen) => {
            let (expr, t) = expression(iter)?;

            match iter.next() {
                Some(Token::RightParen) => Ok((Box::new(Expr::Grouping(t, expr)), t)),
                _ => Err("Expected closing ')'."),
            }
        }
        Some(Token::Minus) => {
            let (expr, t) = primary(iter)?;
            Ok((Box::new(Expr::Unary(t, expr)), t))
        }
        Some(Token::Number(t, f)) => Ok((Box::new(Expr::Literal(*t, f)), *t)),
        _ => Err("Expected an expression."),
    }
}

fn factor<'a>(iter: &mut Peekable<Iter<'a, Token>>) -> Result<(Box<Expr<'a>>, Type), &'a str> {
    let mut left = primary(iter)?;

    while let Some(Token::Star | Token::Slash) = iter.peek() {
        let operator = iter.next().unwrap();
        let right = primary(iter)?;
        if left.1 != right.1 {
            return Err("შეუძლებელია ამ ოპერაციის განხორციელება სხვადასხვა ტიპის რიცხვებზე");
        }
        left = (
            Box::new(Expr::Binary(left.1, left.0, operator, right.0)),
            left.1,
        );
    }

    Ok(left)
}

fn term<'a>(iter: &mut Peekable<Iter<'a, Token>>) -> Result<(Box<Expr<'a>>, Type), &'a str> {
    let mut left = factor(iter)?;

    while let Some(Token::Plus | Token::Minus) = iter.peek() {
        let operator = iter.next().unwrap();
        let right = factor(iter)?;
        if left.1 != right.1 {
            return Err("შეუძლებელია ამ ოპერაციის განხორციელება სხვადასხვა ტიპის რიცხვებზე");
        }
        left = (
            Box::new(Expr::Binary(left.1, left.0, operator, right.0)),
            left.1,
        );
    }

    Ok(left)
}

fn expression<'a>(iter: &mut Peekable<Iter<'a, Token>>) -> Result<(Box<Expr<'a>>, Type), &'a str> {
    term(iter)
}

pub fn parse(tokens: &[Token]) -> Result<Box<Expr>, &str> {
    let mut iter = tokens.iter().peekable();
    let result = expression(&mut iter)?;

    if iter.next().is_some() {
        return Err("Expected an expression.");
    }

    Ok(result.0)
}
