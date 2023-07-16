#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    String(String),
    Boolean(bool),
    Identifier(String),
    V,
    W,
    I,
    F,
    Print,
    Equals,
    Comma,
    EqualEqual,
    Bang,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    And,
    Or,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Return,
    EndOfFile,
    Unknown,
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = code.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => continue,
            '!' => {
                if let Some(&ch) = iter.peek() {
                    match ch {
                        '=' => {
                            iter.next();
                            tokens.push(Token::NotEqual);
                        }
                        _ => tokens.push(Token::Bang),
                    }
                } else {
                    tokens.push(Token::Bang);
                }
            }
            '=' => {
                if let Some(&ch) = iter.peek() {
                    match ch {
                        '=' => {
                            iter.next();
                            tokens.push(Token::EqualEqual);
                        }
                        _ => tokens.push(Token::Equals),
                    }
                } else {
                    tokens.push(Token::Equals);
                }
            }
            ',' => tokens.push(Token::Comma),
            '<' => {
                if let Some(&ch) = iter.peek() {
                    match ch {
                        '=' => {
                            iter.next();
                            tokens.push(Token::LessThanEquals);
                        }
                        _ => tokens.push(Token::LessThan),
                    }
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                if let Some(&ch) = iter.peek() {
                    match ch {
                        '=' => {
                            iter.next();
                            tokens.push(Token::GreaterThanEquals);
                        }
                        _ => tokens.push(Token::GreaterThan),
                    }
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '&' => tokens.push(Token::And),
            '|' => tokens.push(Token::Or),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            '%' => tokens.push(Token::Percent),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '"' => {
                let mut string = String::new();
                while let Some(&ch) = iter.peek() {
                    match ch {
                        '"' => {
                            iter.next();
                            break;
                        }
                        _ => string.push(iter.next().unwrap()),
                    }
                }
                tokens.push(Token::String(string));
            }
            c if c.is_digit(10) => {
                let mut number = String::new();
                number.push(c);
                while let Some(&ch) = iter.peek() {
                    if ch.is_digit(10) {
                        number.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            c if c.is_alphabetic() => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&ch) = iter.peek() {
                    if ch.is_alphabetic()  || ch == '_' {
                        identifier.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "v" => tokens.push(Token::V),
                    "w" => tokens.push(Token::W),
                    "i" => tokens.push(Token::I),
                    "f" => tokens.push(Token::F),
                    "true" => tokens.push(Token::Boolean(true)),
                    "false" => tokens.push(Token::Boolean(false)),
                    "p" => tokens.push(Token::Print),
                    "r" => tokens.push(Token::Return),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            _ => tokens.push(Token::Unknown),
        }
    }

    tokens.push(Token::EndOfFile);
    tokens
}
