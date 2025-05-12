use crate::lexer::Token::{Newline, Number, Print};
use crate::parser::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Number(Type, String),
    Print,
    Newline,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, &str> {
    let mut tokens = vec![];
    let mut iterator = input.chars().peekable();

    while let Some(char) = iterator.next() {
        let token = match char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '0'..='9' => {
                let mut number = String::from(char);

                while let Some(char) = iterator.peek() {
                    match char {
                        '0'..='9' | '.' => {
                            number.push(*char);
                            iterator.next();
                        }
                        _ => break,
                    }
                }
                let num_type: Type = match number.parse::<i32>() {
                    Ok(_) => Type::I32,
                    Err(_) => match number.parse::<f64>() {
                        Ok(_) => Type::F64,
                        Err(_) => return Err("invalid number format."),
                    },
                };
                Number(num_type, number)
            }
            'ა'..='ჰ' => {
                let mut keyword = String::from(char);
                while let Some(char) = iterator.peek() {
                    match char {
                        'ა'..='ჰ' | '0'..='9' => {
                            keyword.push(*char);
                            iterator.next();
                        }
                        _ => break,
                    }
                }
                match keyword.as_str() {
                    "დაბეჭდე" => Print,
                    _ => return Err("Invalid keyword."),
                }
            }
            '\n' => Newline,
            ' ' | '\r' | '\t' => {
                continue;
            }
            _ => {
                return Err("invalid input.");
            }
        };
        tokens.push(token);
    }
    Ok(tokens)
}
