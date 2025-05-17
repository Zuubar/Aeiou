#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Number,
    Print,
    Var,
    Colon,
    Equal,
    EqualEqual,
    Identifier,
    Newline,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub type_: TokenType,
    pub string: String,
}

impl Token {
    pub fn new(type_: TokenType, string: String) -> Token {
        Token { type_, string }
    }
    pub fn from_type(type_: TokenType) -> Token {
        Token {
            type_,
            string: String::new(),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, &str> {
    let mut tokens = vec![];
    let mut iterator = input.chars().peekable();

    while let Some(char) = iterator.next() {
        let token = match char {
            '+' => Token::from_type(TokenType::Plus),
            '-' => Token::from_type(TokenType::Minus),
            '*' => Token::from_type(TokenType::Star),
            '/' => Token::from_type(TokenType::Slash),
            '(' => Token::from_type(TokenType::LeftParen),
            ')' => Token::from_type(TokenType::RightParen),
            ':' => Token::from_type(TokenType::Colon),
            '=' => match iterator.next() {
                Some('=') => Token::from_type(TokenType::EqualEqual),
                _ => Token::from_type(TokenType::Equal),
            },
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

                match number.parse::<i32>() {
                    Ok(_) => Token::new(TokenType::Number, number),
                    Err(_) => match number.parse::<f64>() {
                        Ok(_) => Token::new(TokenType::Number, number),
                        Err(_) => return Err("invalid number format."),
                    },
                }
            }
            'ა'..='ჰ' | 'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::from(char);
                while let Some(char) = iterator.peek() {
                    match char {
                        'ა'..='ჰ' | 'a'..='z' | 'A'..='Z' | '0'..='9' => {
                            identifier.push(*char);
                            iterator.next();
                        }
                        _ => break,
                    }
                }
                match identifier.as_str() {
                    "დაბეჭდე" => Token::from_type(TokenType::Print),
                    "ცვლადი" => Token::new(TokenType::Var, identifier),
                    _ => Token::new(TokenType::Identifier, identifier),
                }
            }
            '\n' => Token::from_type(TokenType::Newline),
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
