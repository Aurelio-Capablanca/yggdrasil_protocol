use crate::structure::expression::Expression;
use crate::structure::operator_tree::Operator;
use std::collections::HashMap;

macro_rules!  hashmap {
    ($ ($key : expr => $val : expr), *) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val); )*
        map
    }};
}

fn process(mut expression_get: &str) {}

fn evaluation_boolean(expression: &Expression<String>, values: &HashMap<String, bool>) -> bool {
    match expression {
        Expression::VAR(def) => *values.get(def).unwrap_or(&false),
        Expression::NOT(def) => !evaluation_boolean(def, values),
        Expression::OR(left, right) => {
            evaluation_boolean(left, values) || evaluation_boolean(right, values)
        }
        Expression::AND(left, right) => {
            evaluation_boolean(left, values) && evaluation_boolean(right, values)
        }
        &_ => false,
    }
}

// pub fn operation_arithmeticals(operator: &Operator<f64>) -> f64 {
//     match operator {
//         Operator::Val(n) => *n,
//         Operator::Sum(a, b) => operation_arithmeticals(a) + operation_arithmeticals(b),
//         Operator::Subtract(a, b) => operation_arithmeticals(a) - operation_arithmeticals(b),
//         Operator::Multiply(a, b) => operation_arithmeticals(a) * operation_arithmeticals(b),
//         Operator::Division(a, b) => operation_arithmeticals(a) / operation_arithmeticals(b),
//         _=> {let data: f64 = 0.0 }
//     }
// }

pub fn test() {
    let variables =
        hashmap!["A".to_string() => true, "B".to_string() => false, "C".to_string() => true];
    // (A AND (NOT B)) OR C
    let expression = Expression::OR(
        Box::new(Expression::AND(
            Box::new(Expression::VAR("A".to_string())),
            Box::new(Expression::NOT(Box::new(Expression::VAR("B".to_string())))),
        )),
        Box::new(Expression::VAR("C".to_string())),
    );
    let result = evaluation_boolean(&expression, &variables);
    print!("The expression given results in: {}", result);
}
