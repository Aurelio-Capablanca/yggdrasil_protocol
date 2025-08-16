use crate::structure::token::Token;

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    Variable(String),
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
            Expression::Number(n ) => *n,
            Expression::Boolean(b) => if *b {1.0} else {0.0},
            _=> { println!("doesnt apply"); return 0.0 }
        }
    }


    pub fn as_boolean(&self) -> bool{
        match self {
            Expression::Number(n) =>  *n != 0.0,
            Expression::Boolean(b) => *b,
            _=> {println!("doesnt apply"); return false}
        }
    }
}