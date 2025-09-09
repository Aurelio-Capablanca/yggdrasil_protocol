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
                println!(
                    "Addition: {} + {} = {}",
                    left_val.get_numeric(),
                    right_val.get_numeric(),
                    left_val.get_numeric() + right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() + right_val.get_numeric())
            }
            Token::Minus => {
                let left_val = mathematics(&left.to_numeric());
                let right_val = mathematics(&right.to_numeric());
                println!(
                    "Addition: {} - {} = {}",
                    left_val.get_numeric(),
                    right_val.get_numeric(),
                    left_val.get_numeric() - right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() - right_val.get_numeric())
            }
            Token::Multiply => {
                let left_val = mathematics(&left.to_numeric());
                let right_val = mathematics(&right.to_numeric());
                println!(
                    "Addition: {} * {} = {}",
                    left_val.get_numeric(),
                    right_val.get_numeric(),
                    left_val.get_numeric() * right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() * right_val.get_numeric())
            }
            Token::Divide => {
                let left_val = mathematics(&left.to_numeric());
                let right_val = mathematics(&right.to_numeric());
                println!(
                    "Addition: {} / {} = {}",
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
                println!(
                    "Equality check: {} == {} = {}",
                    left_val.get_boolean(),
                    right_val.get_boolean(),
                    left_val.get_boolean() == right_val.get_boolean()
                );
                Response::new().define_boolean(left_val.get_boolean() == right_val.get_boolean())
            }
            Token::Convert => {
                convert_bases(&left.as_numbers(), &right.as_numbers());
                Response::new()
            }
            _ => Response::new(),
        },
        _ => {
            println!("Invalid Operation");
            Response::new()
        }
    }
}

pub fn convert_bases(goal_numeric: &f64, base_destiny: &f64) {
    let base = *base_destiny as i64;
    let mut integer_part: Vec<f64> = Vec::new();
    let mut decimal_part: Vec<f64> = Vec::new();
    let limit_decimals = 5;
    let binding = goal_numeric.to_string();
    let splitter: Vec<&str> = binding.split('.').collect();
    let integer_member_str = splitter.first().unwrap_or(&"0").to_string();
    let decimal_member_str = match splitter.get(1) {
        Some(decimal) => decimal.to_string(),
        None => "0".to_string(),
    };
    println!(
        "Integer: {:?} Decimal {:?}",
        integer_member_str, decimal_member_str
    );
    let integer_member = integer_member_str.parse::<i64>().unwrap_or(0);
    let decimal_member = decimal_member_str.parse::<i64>().unwrap_or(0);
    let mut result: i64 = integer_member;
    while result != 0 {
        let remainder = result % base;
        result = result / base;
        println!("Result = {:?}; reminder = {:?}", result, remainder);
    }
}
