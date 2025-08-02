use std::ptr::null;

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParenthesis,
    RParenthesis,
}

fn tokenization(mathematical_sentence: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut numbers_buffer: String = String::new();

    for cases in mathematical_sentence.chars() {
        match cases {
            '0'..='9' | '.' => numbers_buffer.push(cases),
            '+' => {
                if !numbers_buffer.is_empty(){
                    tokens.push(Token::Number(numbers_buffer.parse().unwrap()));
                    numbers_buffer.clear();
                }
                tokens.push(Token::Plus);
            }
            _ => print!("Ignoring Character : {}", cases),
        }
    }

    tokens
}
