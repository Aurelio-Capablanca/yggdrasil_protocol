use crate::core::mathematical_modes::{binary_arithmetics, convert_mode};
use crate::structure::expression::Expression;
use crate::structure::response::Response;
use crate::structure::token::Token;

pub fn do_maths(expression: &Expression) -> Response {
    match expression {
        Expression::Number(n, _) => Response::new().define_numeric(*n),
        Expression::Boolean(b) => Response::new().define_boolean(*b),
        Expression::Hex(_s, _b) => Response::new(),
        Expression::Unary { op, expr } => match op {
            Token::Not => {
                let action = do_maths(&expr.to_boolean());
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
                let left_val = do_maths(&left.to_numeric());
                let right_val = do_maths(&right.to_numeric());
                println!(
                    "Addition: {} + {} = {}",
                    left_val.get_numeric(),
                    right_val.get_numeric(),
                    left_val.get_numeric() + right_val.get_numeric()
                );
                if *left.get_number_or_hex_base() == 2_i64
                    && *right.get_number_or_hex_base() == 2_i64
                {
                    let result = binary_arithmetics::sum_binaries(
                        *left_val.get_numeric(),
                        *right_val.get_numeric(),
                    );
                    println!("{}", result);
                    return Response::new();
                }
                Response::new().define_numeric(left_val.get_numeric() + right_val.get_numeric())
            }
            Token::Minus => {
                let left_val = do_maths(&left.to_numeric());
                let right_val = do_maths(&right.to_numeric());
                println!(
                    "Substraction: {} - {} = {}",
                    left_val.get_numeric(),
                    right_val.get_numeric(),
                    left_val.get_numeric() - right_val.get_numeric()
                );
                if *left.get_number_or_hex_base() == 2_i64 && *right.get_number_or_hex_base() == 2_i64 {
                    let result = binary_arithmetics::subtract_binaries(*left_val.get_numeric(), *right_val.get_numeric());
                    println!("{}",result);
                    return Response::new();
                }
                Response::new().define_numeric(left_val.get_numeric() - right_val.get_numeric())
            }
            Token::Multiply => {
                let left_val = do_maths(&left.to_numeric());
                let right_val = do_maths(&right.to_numeric());
                println!(
                    "Multiplication: {} * {} = {}",
                    left_val.get_numeric(),
                    right_val.get_numeric(),
                    left_val.get_numeric() * right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() * right_val.get_numeric())
            }
            Token::Divide => {
                let left_val = do_maths(&left.to_numeric());
                let right_val = do_maths(&right.to_numeric());
                println!(
                    "Division: {} / {} = {}",
                    left_val.get_numeric(),
                    right_val.get_numeric(),
                    left_val.get_numeric() / right_val.get_numeric()
                );
                Response::new().define_numeric(left_val.get_numeric() / right_val.get_numeric())
            }
            //booleans too!
            Token::Equals => {
                let left_val = do_maths(left);
                let right_val = do_maths(right);
                println!(
                    "Equality check: {} == {} = {}",
                    left_val.get_boolean(),
                    right_val.get_boolean(),
                    left_val.get_boolean() == right_val.get_boolean()
                );
                Response::new().define_boolean(left_val.get_boolean() == right_val.get_boolean())
            }
            //For Convert Mode usage
            Token::Convert => {
                let base = left.get_number_or_hex_base();
                if base <= &10_i64 {
                    convert_mode::convert_bases(&left.as_numbers(), &base, &right.as_numbers());
                } else {
                    convert_mode::convert_hex_bases(
                        &left.get_hex().to_string(),
                        /*&base*/
                        &right.as_numbers(),
                    );
                }
                Response::new()
            }
            _ => Response::new(),
        },
        _ => Response::new(),
    }
}
