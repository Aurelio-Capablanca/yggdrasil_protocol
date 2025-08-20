use crate::structure::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    //Variable(String),
    Binary {
        op: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: Token,
        expr: Box<Expression>,
    },
    Assignment{
        value : f64,
        variable: String
    }
}

impl Expression{

    pub fn as_numbers(&self) -> f64{
        match self {
            Expression::Number(n ) => {println!("You hit here! NUMBERS as_numbers "); *n },
            Expression::Boolean(b) => {println!("You hit here! BOOLEAN as_numbers "); if *b { 1.0 } else { 0.0 } },
            _=> 0.0
        }
    }

    pub fn to_numeric(&self) -> Expression{
        match self {
            Expression::Boolean(b) => { println!("You hit here! to_numeric "); Expression::Number(if *b { 1.0 } else { 0.0 }) },
            _=> self.clone()
        }
    }

    pub fn as_boolean(&self) -> bool{
        match self {
            Expression::Number(n) =>  *n != 0.0,
            Expression::Boolean(b) => *b,
            _=> false
        }
    }


    pub fn to_boolean(&self) -> Expression {
        match self {
            Expression::Number(n) => Expression::Boolean(*n != 0.0),
            _=> self.clone()
        }
    }
}