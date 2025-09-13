use crate::structure::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64, i64),
    Boolean(bool),
    Hex(String, i64),
    _Variable(String),
    Binary {
        op: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: Token,
        expr: Box<Expression>,
    },
}

impl Expression {
    pub fn as_numbers(&self) -> f64 {
        match self {
            Expression::Number(n, _) => *n,
            Expression::Boolean(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    pub fn get_number_or_hex_base(&self) -> &i64 {
        match self {
            Expression::Number(_, b) => b,
            Expression::Hex(_, b) => b,
            _ => &10_i64,
        }
    }

    pub fn to_numeric(&self) -> Expression {
        match self {
            Expression::Boolean(b) => Expression::Number(if *b { 1.0 } else { 0.0 }, 10),
            _ => self.clone(),
        }
    }

    pub fn as_boolean(&self) -> bool {
        match self {
            Expression::Number(n, _) => *n != 0.0,
            Expression::Boolean(b) => *b,
            _ => false,
        }
    }

    pub fn to_boolean(&self) -> Expression {
        match self {
            Expression::Number(n, _) => Expression::Boolean(*n != 0.0),
            _ => self.clone(),
        }
    }
}
