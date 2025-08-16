//use crate::structure::expression::Expression;
use crate::structure::calculus_structure::Expression;
use crate::structure::operator_tree::Operator;
use crate::structure::token::Token;
use std::collections::HashMap;
use crate::structure::response::response;

macro_rules!  hashmap {
    ($ ($key : expr => $val : expr), *) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val); )*
        map
    }};
}

// fn evaluation_boolean(expression: &Expression<String>, values: &HashMap<String, bool>) -> bool {
//     match expression {
//         Expression::VAR(def) => *values.get(def).unwrap_or(&false),
//         Expression::NOT(def) => !evaluation_boolean(def, values),
//         Expression::OR(left, right) => {
//             evaluation_boolean(left, values) || evaluation_boolean(right, values)
//         }
//         Expression::AND(left, right) => {
//             evaluation_boolean(left, values) && evaluation_boolean(right, values)
//         }
//         &_ => false,
//     }
// }

pub fn ar_operations(operator: &Operator<f64>) -> f64 {
    match operator {
        Operator::Val(n) => *n,
        Operator::Sum(a, b) => ar_operations(a) + ar_operations(b),
        Operator::Subtract(a, b) => ar_operations(a) - ar_operations(b),
        Operator::Multiply(a, b) => ar_operations(a) * ar_operations(b),
        Operator::Division(a, b) => ar_operations(a) / ar_operations(b),
        _ => return 0.0,
    }
}

pub fn arithmetics(expression: &Expression) -> response {
    match expression {
        Expression::Number(n) => {
           // print!("{:?}",n);
            response::new().set_numeric(*n)
        }
        //Fix here!
        Expression::Binary { op, left, right } => match op {
            Token::Plus => { arithmetics(left) + arithmetics(right) },
            Token::Minus => arithmetics(left) - arithmetics(right),
            Token::Multiply => arithmetics(left) * arithmetics(right),
            Token::Divide => arithmetics(left) / arithmetics(right),
            _ => response::new(),
        },
        _ => {
            println!("Invalid Operation");
           //
            response::new()
        }
    }
}

// pub fn test() {
//     let variables =
//         hashmap!["A".to_string() => true, "B".to_string() => false, "C".to_string() => true];
//     // (A AND (NOT B)) OR C
//     let expression = Expression::OR(
//         Box::new(Expression::AND(
//             Box::new(Expression::VAR("A".to_string())),
//             Box::new(Expression::NOT(Box::new(Expression::VAR("B".to_string())))),
//         )),
//         Box::new(Expression::VAR("C".to_string())),
//     );
//     let result = evaluation_boolean(&expression, &variables);
//     print!("The expression given results in: {}", result);
// }
