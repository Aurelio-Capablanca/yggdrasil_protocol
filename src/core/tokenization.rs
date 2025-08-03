use crate::structure::operator_tree::Operator;

#[derive(Debug)]
pub enum Token {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,

    // Comparison
    Equals,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Logical
    And, // &&
    Or,  // ||
    Not, // !

    // Values
    True,
    False,
    Number(f64),

    // Punctuation
    LParenthesis,
    RParenthesis,
}

pub fn tokenization(mathematical_sentence: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut numbers_buffer: String = String::new();
    let mut operator: Operator<f64>;

    for cases in mathematical_sentence.chars() {
        match cases {
            '0'..='9' | '.' => numbers_buffer.push(cases),
            '+' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
                tokens.push(Token::Plus);
            }
            '-' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
                tokens.push(Token::Minus)
            }
            '*' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
                tokens.push(Token::Multiply);
            }
            '/' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
                tokens.push(Token::Divide);
            }
            '(' => tokens.push(Token::LParenthesis),
            ')' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
                tokens.push(Token::RParenthesis);
            }
            '&' | '|' | '=' | '!' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
                let mut chars = mathematical_sentence.chars().peekable();
                while let Some(cases) = chars.next() {
                    match cases {
                        '&' => {
                            if let Some('&') = chars.peek() {
                                chars.next();
                                tokens.push(Token::And);
                            }
                        }
                        '|' => {
                            if let Some('|') = chars.peek() {
                                chars.next();
                                tokens.push(Token::Or);
                            }
                        }
                        '!' => {
                            if let Some('=') = chars.peek(){
                                chars.next();
                                tokens.push(Token::NotEqual);
                            } else {
                              tokens.push(Token::Not);   
                            }
                        }
                        '=' => {
                            if let Some('=') = chars.peek(){
                                chars.next();
                                tokens.push(Token::Equals)
                            }
                        }
                        _ => {
                            print!("Do nothing! at? {:?}",cases)
                        }
                    }
                }
            }
            ' ' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
            }
            _ => print!("Ignoring Character : {}", cases),
        }
    }
    if !numbers_buffer.is_empty() {
        tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
    }
    tokens.iter().for_each(|data| print!("{:?},", data));
    tokens
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Operator<f64> {
    let parsed = parse_add_sub(tokens);
    print!("{:?}", parsed);
    parsed
}

pub fn parse_add_sub(tokens: &mut Vec<Token>) -> Operator<f64> {
    let mut node = parse_mul_div(tokens);
    while let Some(token) = tokens.first() {
        match token {
            Token::Plus | Token::Minus => {
                let operator = tokens.remove(0);
                let right = parse_mul_div(tokens);
                node = match operator {
                    Token::Plus => Operator::Sum(Box::new(node), Box::new(right)),
                    Token::Minus => Operator::Subtract(Box::new(node), Box::new(right)),
                    _ => return node,
                }
            }
            _ => return node,
        }
    }
    node
}

pub fn parse_mul_div(tokens: &mut Vec<Token>) -> Operator<f64> {
    let mut node = parse_primary(tokens);

    while let Some(token) = tokens.first() {
        match token {
            Token::Multiply | Token::Divide => {
                let operation = tokens.remove(0);
                let right = parse_primary(tokens);
                node = match operation {
                    Token::Multiply => Operator::Multiply(Box::new(node), Box::new(right)),
                    Token::Divide => Operator::Division(Box::new(node), Box::new(right)),
                    _ => return node,
                }
            }
            _ => return node,
        }
    }

    node
}

fn parse_primary(tokens: &mut Vec<Token>) -> Operator<f64> {
    match tokens.remove(0) {
        Token::Number(n) => Operator::Val(n),
        Token::LParenthesis => {
            let node = parse_expression(tokens);
            match tokens.remove(0) {
                Token::RParenthesis => node,
                _ => panic!("Expected closing parenthesis"),
            }
        }
        other => panic!("Unexpected token: {:?}", other),
    }
}
