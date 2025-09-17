use crate::core::mathematical_modes::{binary_arithmetics, convert_mode};
use crate::structure::expression::Expression;
use crate::structure::response::Response;
use crate::structure::token::Token;

pub fn do_maths(expression: &Expression) -> Response {
    match expression {
        Expression::Number(n, _) => Response::new().define_numeric(*n),
        Expression::Boolean(b) => Response::new().define_boolean(*b),
        Expression::Hex(s, _b) => Response::new().define_string(s.to_string()),
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
                    let result_sum = binary_arithmetics::sum_binaries(
                        do_maths(&left).get_string(),
                        do_maths(&right).get_string(),
                    );
                    println!("{}", result_sum);
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
                if *left.get_number_or_hex_base() == 2_i64
                    && *right.get_number_or_hex_base() == 2_i64
                {
                    let result_subtract = binary_arithmetics::subtract_binaries(
                        do_maths(&left).get_string(),
                        do_maths(&right).get_string(),
                    );
                    println!("{}", result_subtract);
                    return Response::new().define_string(result_subtract);
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
                if *left.get_number_or_hex_base() == 2_i64
                    && *right.get_number_or_hex_base() == 2_i64
                {
                    let product_result = binary_arithmetics::multiply_binaries(
                        do_maths(&left).get_string().to_string(),
                        do_maths(&right).get_string().to_string(),
                    );
                    println!("{}", product_result);
                    return Response::new().define_string(product_result);
                }
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
                if *left.get_number_or_hex_base() == 2_i64
                    && *right.get_number_or_hex_base() == 2_i64
                {
                    let (quotient, reminder) = binary_arithmetics::divide_binaries(
                        do_maths(&left).get_string(),
                        do_maths(&right).get_string(),
                        4,
                    );
                    println!("{}, {}", quotient, reminder);
                    return Response::new().define_string(quotient);
                }
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
