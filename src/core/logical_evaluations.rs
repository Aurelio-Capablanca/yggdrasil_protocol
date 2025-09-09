use crate::structure::calculus_structure::Expression;
use crate::structure::response::Response;
use crate::structure::token::Token;

pub fn mathematics(expression: &Expression) -> Response {
    match expression {
        Expression::Number(n) => Response::new().define_numeric(*n),
        Expression::Boolean(b) => Response::new().define_boolean(*b),
        Expression::Unary { op, expr } => match op {
            Token::Not => {
                let action = mathematics(&expression.to_boolean());
                println!(" NOT : !{}", action.get_boolean());
                Response::new().define_boolean(!action.get_boolean())
            }
            _ => {
                println!("Unknown Unary Operator");
                Response::new()
            }
        },
        Expression::Binary { op, left, right } => match op {
            Token::Plus => {
                let left_val = mathematics(&left.to_numeric());
                let right_val = mathematics(&right.to_numeric());
                println!("Addition: {} + {} = {}",
                         left_val.get_numeric(),
                         right_val.get_numeric(),
                         left_val.get_numeric() + right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() + right_val.get_numeric())
            }
            Token::Minus => {
                let left_val = mathematics(&left.to_numeric());
                let right_val = mathematics(&right.to_numeric());
                println!("Addition: {} - {} = {}",
                         left_val.get_numeric(),
                         right_val.get_numeric(),
                         left_val.get_numeric() - right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() - right_val.get_numeric())
            }
            Token::Multiply => {
                let left_val = mathematics(&left.to_numeric());
                let right_val = mathematics(&right.to_numeric());
                println!("Addition: {} * {} = {}",
                         left_val.get_numeric(),
                         right_val.get_numeric(),
                         left_val.get_numeric() * right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() * right_val.get_numeric())
            }
            Token::Divide => {
                let left_val = mathematics(&left.to_numeric());
                let right_val = mathematics(&right.to_numeric());
                println!("Addition: {} / {} = {}",
                         left_val.get_numeric(),
                         right_val.get_numeric(),
                         left_val.get_numeric() / right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() / right_val.get_numeric())
            }
            //booleans too!
            Token::Equals => {
                let left_val = mathematics(left);
                let right_val = mathematics(right);
                println!("Equality check: {} == {} = {}",
                         left_val.get_boolean(),
                         right_val.get_boolean(),
                         left_val.get_boolean() == right_val.get_boolean()
                );
                Response::new().define_boolean(left_val.get_boolean() == right_val.get_boolean())
            }
            _ => Response::new(),
        },
        _ => {
            println!("Invalid Operation");
            Response::new()
        }
    }
}
