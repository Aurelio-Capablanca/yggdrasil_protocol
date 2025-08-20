use crate::structure::token::Token;

#[derive(Debug)]
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
            Expression::Number(n ) => *n,
            Expression::Boolean(b) => if *b {1.0} else {0.0},
            _=> { println!("doesnt apply"); 0.0 }
        }
    }

    pub fn convert_to_numeric(&self) -> Expression{
        match self {
            Expression::Boolean(b) => Expression::Number(if *b {1.0} else {0.0}),
            _=> {println!("doesn't apply"); Expression::Number(self.as_numbers())}
        }
    }

    pub fn as_boolean(&self) -> bool{
        match self {
            Expression::Number(n) =>  *n != 0.0,
            Expression::Boolean(b) => *b,
            _=> {println!("doesnt apply"); false}
        }
    }


    pub fn convert_to_boolean(&self) -> Expression {
        match self {
            Expression::Number(n) => Expression::Boolean(*n != 0.0),
            _=> Expression::Boolean(self.as_boolean())
        }
    }
}