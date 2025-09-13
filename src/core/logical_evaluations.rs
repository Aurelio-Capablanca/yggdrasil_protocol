use crate::structure::expression::Expression;
use crate::structure::response::Response;
use crate::structure::token::Token;

pub fn mathematics(expression: &Expression) -> Response {
    match expression {
        Expression::Number(n, _) => Response::new().define_numeric(*n),
        Expression::Boolean(b) => Response::new().define_boolean(*b),
        Expression::Hex(_s, _b) => Response::new(),
        Expression::Unary { op, expr } => match op {
            Token::Not => {
                let action = mathematics(&expr.to_boolean());
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
                let base = left.get_number_or_hex_base();
                if base <= &10_i64 {
                    convert_bases(&left.as_numbers(), &base, &right.as_numbers());
                } else {
                }
                Response::new()
            }
            _ => Response::new(),
        },
        _ => Response::new(),
    }
}

pub fn validate_number_for_base(goal_validate: &str, base_origin: &i64) -> bool {
    for element in goal_validate.chars() {
        if element == '.' {
            continue;
        }
        match element.to_digit(10) {
            Some(digit) => {
                let value = i64::from(digit);
                if value >= *base_origin {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }
    true
}

pub fn convert_bases(goal_numeric: &f64, base_origin: &i64, base_destiny: &f64) -> f64 {
    let base = *base_destiny as i64;
    let eval = validate_number_for_base(&goal_numeric.to_string(), &base_origin);
    if eval == false {
        println!("Mismatched!!!");
        return 0.0;
    }
    let mut integer_part: Vec<i64> = Vec::new();
    let mut decimal_part: Vec<i64> = Vec::new();
    let limit_decimals = 5;
    let binding = goal_numeric.to_string();
    let splitter: Vec<&str> = binding.split('.').collect();
    let integer_member_str = splitter.first().unwrap_or(&"0").to_string();
    let decimal_member_str = match splitter.get(1) {
        Some(decimal) => format!("0.{}", decimal.to_string()),
        None => "0".to_string(),
    };
    println!(
        "Integer: {:?} Decimal {:?}",
        integer_member_str, decimal_member_str
    );
    let integer_member = integer_member_str.parse::<i64>().unwrap_or(0);
    let mut decimal_member = decimal_member_str.parse::<f64>().unwrap_or(0.0);
    //do Integer Part
    let mut result: i64 = integer_member;
    while result != 0 {
        let remainder = result % base;
        integer_part.push(remainder);
        result = result / base;
        println!("Result = {:?}; reminder = {:?}", result, remainder);
    }
    integer_part.reverse();
    //do Decimal Part
    for _ in 0..limit_decimals {
        let fraction = decimal_member * base_destiny;
        let number = fraction.floor() as i64;
        decimal_part.push(number);
        decimal_member = fraction - fraction.floor();
        if decimal_member == 0.0 {
            break;
        }
        println!("Result = {:?}; reminder = {:?}", result, fraction);
    }
    //Set Order
    let final_integer = integer_part
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let final_decimal = decimal_part
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    let format_number = format!("{}.{}", final_integer, final_decimal);
    println!("Result : {}", format_number);
    format_number.parse::<f64>().unwrap_or(0.0)
}

fn convert_hex_bases(goal_hex: &String, base_origin: &i64, base_destiny: &f64) {}
