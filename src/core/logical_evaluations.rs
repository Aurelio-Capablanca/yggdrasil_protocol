use crate::structure::expression::Expression;
use std::collections::HashMap;

macro_rules!  hashmap {
    ($ ($key : expr => $val : expr), *) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val); )*
        map
    }};
}

fn process(mut expression_get: &str) {}

fn eval(expression: &Expression<String>, values: &HashMap<String, bool>) -> bool {
    match expression { 
        Expression::VAR(def) => *values.get(def).unwrap_or(&false),
        Expression::NOT(def) => !eval(def, values),
        Expression::OR(left, right) => eval(left, values) || eval(right, values),
        Expression::AND(left, right) => eval(left, values) && eval(right, values),
        &_ => false
    }
}

pub fn test() {
    let variables = hashmap!["A".to_string() => true, "B".to_string() => false, "C".to_string() => true];
    // (A AND (NOT B)) OR C
    let expression = Expression::OR(
        Box::new(Expression::AND(
            Box::new(Expression::VAR("A".to_string())),
            Box::new(Expression::NOT(Box::new(Expression::VAR("B".to_string())))),
        )),
        Box::new(Expression::VAR("C".to_string())),
    );
    let result = eval(&expression, &variables);
    print!("The expression given results in: {}",result);
}
