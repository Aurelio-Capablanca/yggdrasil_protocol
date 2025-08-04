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

fn set_numbers(numbers_buffer: &mut String, tokens: &mut Vec<Token>) {
    if !numbers_buffer.is_empty() {
        tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
        numbers_buffer.clear();
    }
}

pub fn tokenization(mathematical_sentence: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut numbers_buffer: String = String::new();

    let mut chars = mathematical_sentence.chars().peekable();
    while let Some(cases) = chars.next() {
        match cases {
            '0'..='9' | '.' => numbers_buffer.push(cases),
            '+' => {
                set_numbers(&mut numbers_buffer, &mut tokens);
                tokens.push(Token::Plus);
            }
            '-' => {
                set_numbers(&mut numbers_buffer, &mut tokens);
                tokens.push(Token::Minus)
            }
            '*' => {
                set_numbers(&mut numbers_buffer, &mut tokens);
                tokens.push(Token::Multiply);
            }
            '/' => {
                set_numbers(&mut numbers_buffer, &mut tokens);
                tokens.push(Token::Divide);
            }
            '(' => tokens.push(Token::LParenthesis),
            ')' => {
                set_numbers(&mut numbers_buffer, &mut tokens);
                tokens.push(Token::RParenthesis);
            }
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
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::NotEqual);
                } else {
                    tokens.push(Token::Not);
                }
            }
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Equals)
                }
            }
            '>'/*Greater than*/ => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                } else { 
                    tokens.push(Token::Greater);
                }
            }
            '<'/*less than*/ => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::LessEqual);
                } else {
                    tokens.push(Token::Less);
                }
            }
            ' ' => {
                if !numbers_buffer.is_empty() {
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
            }
            _ => {
                print!("Do nothing! at? {:?}", cases)
            }
        }
    }

    if !numbers_buffer.is_empty() {
        tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
    }
    tokens.iter().for_each(|data| print!("{:?},", data));
    tokens
}
//
// pub fn parse_expression(tokens: &mut Vec<Token>) -> Operator<f64> {
//     let parsed = parse_boolean(tokens);
//     print!("{:?}", parsed);
//     parsed
// }
//
// fn parse_boolean(tokens: &mut Vec<Token>) -> Operator<f64> {
//     let mut node = parse_add_sub(tokens);
//
//     while let Some(token) = tokens.first() {
//         match token {
//             Token::And | Token::Or | Token::Equals | Token::NotEqual | Token::Not => {
//                 let operator = tokens.remove(0);
//                 let right = parse_boolean(tokens);
//                 node = match operator {
//                     Token::And => Operator::And(Box::new(node), Box::new(right)),
//                     Token::Or => Operator::Or(Box::new(node), Box::new(right)),
//                     Token::Equals => Operator::Equals(Box::new(node), Box::new(right)),
//                     Token::NotEqual => Operator::NotEquals(Box::new(node), Box::new(right)),
//                     Token::Not => Operator::Not(Box::new(node)),
//                     _=> return node,
//                 }
//             }
//             _ => return node,
//         }
//     }
//
//     node
// }
//
// fn parse_add_sub(tokens: &mut Vec<Token>) -> Operator<f64> {
//     let mut node = parse_mul_div(tokens);
//     while let Some(token) = tokens.first() {
//         match token {
//             Token::Plus | Token::Minus => {
//                 let operator = tokens.remove(0);
//                 let right = parse_mul_div(tokens);
//                 node = match operator {
//                     Token::Plus => Operator::Sum(Box::new(node), Box::new(right)),
//                     Token::Minus => Operator::Subtract(Box::new(node), Box::new(right)),
//                     _ => return node,
//                 }
//             }
//             _ => return node,
//         }
//     }
//     node
// }
//
// fn parse_mul_div(tokens: &mut Vec<Token>) -> Operator<f64> {
//     let mut node = parse_primary(tokens);
//
//     while let Some(token) = tokens.first() {
//         match token {
//             Token::Multiply | Token::Divide => {
//                 let operation = tokens.remove(0);
//                 let right = parse_primary(tokens);
//                 node = match operation {
//                     Token::Multiply => Operator::Multiply(Box::new(node), Box::new(right)),
//                     Token::Divide => Operator::Division(Box::new(node), Box::new(right)),
//                     _ => return node,
//                 }
//             }
//             _ => return node,
//         }
//     }
//
//     node
// }
//
// fn parse_primary(tokens: &mut Vec<Token>) -> Operator<f64> {
//     match tokens.remove(0) {
//         Token::Number(n) => Operator::Val(n),
//         Token::LParenthesis => {
//             let node = parse_expression(tokens);
//             match tokens.remove(0) {
//                 Token::RParenthesis => node,
//                 _ => panic!("Expected closing parenthesis"),
//             }
//         }
//         other => panic!("Unexpected token: {:?}", other),
//     }
// }
