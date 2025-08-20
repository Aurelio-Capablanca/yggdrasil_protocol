use crate::structure::calculus_structure::Expression;
use crate::structure::token::Token;
use crate::structure::response::Response;

macro_rules!  hashmap {
    ($ ($key : expr => $val : expr), *) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val); )*
        map
    }};
}

pub fn mathematics(expression: &Expression) -> Response {
    match expression {
        Expression::Number(n) => Response::new().define_numeric(*n),
        Expression::Boolean(b ) => Response::new().define_boolean(*b),
        Expression::Unary {op, expr} => match op {
            Token::Not => {
                println!("NOT({:?})",expr.to_boolean());
                Response::new().define_boolean(!mathematics(&expr.to_boolean()).get_boolean())
            },
            _=> { println!("Getting out of Unary!!!"); Response::new() }
        },
        Expression::Binary { op, left, right } => match op {
            Token::Plus => {
                println!("{:?} + {:?}", mathematics(&left.to_numeric()).get_numeric(), mathematics(&right.to_numeric()).get_numeric());
                Response::new().define_numeric(mathematics(&left.to_numeric()).get_numeric() + mathematics(&right.to_numeric()).get_numeric())
            },
            Token::Minus => {
                println!("{:?} - {:?}",mathematics(left).get_numeric(),mathematics(right).get_numeric());
                Response::new().define_numeric(mathematics(left).get_numeric() - mathematics(right).get_numeric())
            },
            Token::Multiply => {
                println!("{:?} * {:?}",mathematics(left).get_numeric(),mathematics(right).get_numeric());
                Response::new().define_numeric(mathematics(left).get_numeric() * mathematics(right).get_numeric())
            },
            Token::Divide => {
                println!("{:?} / {:?}",mathematics(left).get_numeric(),mathematics(right).get_numeric());
                Response::new().define_numeric(mathematics(left).get_numeric() / mathematics(right).get_numeric())
            },
            //booleans too!
            Token::Equals => {
                println!("{:?} == {:?}",mathematics(left).get_numeric(),mathematics(right).get_numeric());
                Response::new().define_boolean(mathematics(left).get_boolean() == mathematics(right).get_boolean())
            },
            _ => Response::new(),
        },
        _ => {
            println!("Invalid Operation");
            Response::new()
        }
    }
}
