use crate::structure::token::Token;

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
            'T' => tokens.push(Token::Boolean(true)),
            'F' => tokens.push(Token::Boolean(false)),
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
    tokens.iter().for_each(|data| println!("{:?},", data));
    println!("\n");
    tokens
}
