mod structure;
mod core;

use crate::core::{logical_evaluations, parse_sentences, tokenization};


fn main() {
    println!("Hello, world!");
    //logical_evaluations::test();
    parse_sentences::parse_strings_to_expression(&"(A AND (NOT B)) OR C".to_string());
    print!("\n");
    parse_sentences::parse_strings_to_expression(&"12 DIV 2 * 3 -18 / ( (17 MOD 3)/2 * 7 – 3) / ( 2 *2 - 8 ) + 15 DIV 3".to_string());
    print!("\n");
    let mut tokens = tokenization::tokenization("( 2 *2 - 8 ) + 15");
    print!("\n");
    let operator = tokenization::parse_expression(&mut tokens);
    print!("\n");
    let result = logical_evaluations::operation_arithmeticals(&operator);
    print!("Final Result : {}",result);
}
