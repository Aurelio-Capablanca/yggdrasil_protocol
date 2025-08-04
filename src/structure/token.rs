#[derive(Debug, PartialEq)]
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
    
    Number(f64),

    // Punctuation
    LParenthesis,
    RParenthesis,
}