use crate::structure::expression::Expression;
use crate::structure::response::Response;
use crate::structure::token::Token;
use lazy_static::lazy_static;
use std::collections::HashMap;

macro_rules ! hashmap {
( $ ( $ key: expr => $ val : expr), * ) => {{
let mut map =::std::collections::HashMap::new();
$ (map.insert( $ key, $ val); ) *
map
}};
}

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
                    convert_hex_bases(&left.get_hex().to_string(), &base, &right.as_numbers());
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

fn parse_int_to_base(num_str: &str, base: i64) -> Option<i64> {
    let mut value: i64 = 0;
    for chars in num_str.chars() {
        let digit = chars.to_digit(10)? as i64;
        if digit >= base {
            return None;
        }
        value = value * base + digit;
    }
    Some(value)
}

fn parse_faction_to_base(frac_str: &str, base: i64) -> Option<f64> {
    let mut value: f64 = 0.0;
    let mut base_factions: f64 = base as f64;
    for chars in frac_str.chars() {
        let digit = chars.to_digit(10)? as i64;
        if digit >= base {
            return None;
        }
        value += (digit as f64) / base_factions;
        base_factions *= base as f64;
    }
    Some(value)
}

pub fn convert_bases(goal_numeric: &f64, base_origin: &i64, base_destiny: &f64) -> f64 {
    let base = *base_destiny as i64;
    if base_origin == &10_i64 && base_destiny == &16_f64 {
        println!("Go from Decimal To Hex");
        decimal_to_hex(goal_numeric, base_origin, base_destiny);
        return 0.0;
    }
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
        Some(decimal) => decimal.to_string(),
        None => "0".to_string(),
    };

    println!(
        "Integer: {:?} Decimal {:?}",
        integer_member_str, decimal_member_str
    );

    let integer_member = parse_int_to_base(&integer_member_str, *base_origin).unwrap_or(0); //integer_member_str.parse::<i64>().unwrap_or(0);
    let mut decimal_member =
        parse_faction_to_base(&decimal_member_str, *base_origin).unwrap_or(0.0); //decimal_member_str.parse::<f64>().unwrap_or(0.0);    
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

lazy_static! {    
    static ref HEX_TO_BINARY: HashMap<String, i64> = hashmap!(
        "0".to_string() => 0000_i64,
        "1".to_string() => 1_i64,
        "2".to_string() => 2_i64,
        "3".to_string() => 3_i64,
        "4".to_string() => 4_i64,
        "5".to_string() => 5_i64,
        "6".to_string() => 6_i64,
        "7".to_string() => 7_i64,
        "8".to_string() => 8_i64,
        "9".to_string() => 9_i64,
        "A".to_string() => 10_i64,
        "B".to_string() => 11_i64
    );
}

fn hex_to_decimal_formula(mut result: i64, target: String) -> Option<String> {
    for chars in target.chars() {
        let numerical = i64::from(chars.to_digit(16).unwrap_or(0));
        result = result * 16 + numerical;
        println!("Result : {}", result);
    }
    Some(result.to_string())
}

fn decimal_to_hex(goal_decimal: &f64, base_origin: &i64, base_destiny: &f64) {
   // split here for Decimal point
    let hex_digits = "0123456789ABCDEF".chars().collect::<Vec<char>>();
    let mut result: Vec<char> = Vec::new();

    let binding = goal_decimal.to_string();
    let splitter: Vec<&str> = binding.split('.').collect();
    let interger_part: String = splitter.first().unwrap_or(&"0").to_string();
    let decimal_part: String = match splitter.get(1) {
        Some(dec) => dec.to_string(),
        None => "0".to_string(),
    };
    let mut integer_member = interger_part.parse::<i64>().unwrap_or(0);
    let mut decimal_member= decimal_part.parse::<i64>().unwrap_or(0);

    while integer_member > 0 {
        let reminder = (integer_member % 16) as usize;
        result.push(hex_digits[reminder]);
        integer_member /= 16;
    }
    let test = result.into_iter().rev().collect::<Vec<char>>();
    test.iter().for_each(|x| println!("{}",x));
}

fn convert_hex_bases(goal_hex: &String, base_origin: &i64, base_destiny: &f64) -> Option<i64> {
    println!("{}", goal_hex);
    let result = 0i64;
    if *base_destiny != 10_f64 {
        //println!("You're trapped here! ")
    }
    println!("Go from Hex to Decimal");
    let splitter: Vec<&str> = goal_hex.split('.').collect();
    let interger_part: String = splitter.first().unwrap_or(&"0").to_string();
    let decimal_part: String = match splitter.get(1) {
        Some(dec) => dec.to_string(),
        None => "0".to_string(),
    };
    let integer_member = hex_to_decimal_formula(result, interger_part).unwrap_or("0".to_string());
    let decimal_member = hex_to_decimal_formula(result, decimal_part).unwrap_or("0".to_string());
    println!(
        "HEX to Decimal ends in: {}.{}",
        integer_member, decimal_member
    );
    Some(0_i64)
}
