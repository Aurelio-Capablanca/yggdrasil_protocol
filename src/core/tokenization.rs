use crate::structure::token::Token;

fn set_numbers(numbers_buffer: &mut String, tokens: &mut Vec<Token>, mode: &u32) {
    if numbers_buffer.is_empty() {
        return;
    }
    if numbers_buffer
        .chars()
        .all(|x| x.is_ascii_digit() || x.eq_ignore_ascii_case(&'.'))
        && *mode != 2_u32
    {
        tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
    } else {
        tokens.push(Token::Strings(numbers_buffer.to_string()))
    }
    numbers_buffer.clear();
}

pub fn tokenization(mathematical_sentence: &str, mode: u32) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut numbers_buffer: String = String::new();

    let mut chars = mathematical_sentence.chars().peekable();
    while let Some(cases) = chars.next() {
        match cases {
            '0'..='9' | '.' => numbers_buffer.push(cases),
            'a' ..='z' => numbers_buffer.push(cases),
            'T' => tokens.push(Token::Boolean(true)),
            'F' => tokens.push(Token::Boolean(false)),
            '\'' => {
                chars.next();
                set_numbers(&mut numbers_buffer, &mut tokens, &mode);
                let mut base_str = String::new();
                while let Some(&nexts) = chars.peek(){
                    if nexts.is_ascii_digit() {
                        base_str.push(nexts);
                        chars.next();
                    } else {break;}
                }
                let base_num = base_str.parse::<i64>().unwrap();
                tokens.push(Token::Base(base_num));
            },
            '+' => {
                set_numbers(&mut numbers_buffer, &mut tokens, &mode);
                tokens.push(Token::Plus);
            }
            '-' => {
                set_numbers(&mut numbers_buffer, &mut tokens, &mode);
                if let Some('>') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Convert)
                } else {
                    tokens.push(Token::Minus)
                }
            }
            '*' => {
                set_numbers(&mut numbers_buffer, &mut tokens, &mode);
                tokens.push(Token::Multiply);
            }
            '/' => {
                set_numbers(&mut numbers_buffer, &mut tokens, &mode);
                tokens.push(Token::Divide);
            }
            '(' => tokens.push(Token::LParenthesis),
            ')' => {
                set_numbers(&mut numbers_buffer, &mut tokens, &mode);
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
                }  else {
                    tokens.push(Token::Less);
                }
            }
            ' ' => {
                set_numbers(&mut numbers_buffer, &mut tokens, &mode)
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
