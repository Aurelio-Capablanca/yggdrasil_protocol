#[derive(Debug, PartialEq, Clone)]
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
    
    //Values
    Number(f64),
    Boolean(bool),

    // Punctuation
    LParenthesis,
    RParenthesis,
}