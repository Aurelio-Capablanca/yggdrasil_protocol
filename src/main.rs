mod structure;
mod core;

use crate::core::{logical_evaluations,  tokenization, tree_generator};


fn main() {
    println!("Hello, world!");
    //logical_evaluations::test();
    let mut token_one = tokenization::tokenization("( 2 * 2 - 8 ) + 15 && 9 + 9");
    let operator = tree_generator::parse_expression(&mut token_one);
    println!("{:?}",operator);
    let mut token_two = tokenization::tokenization("( 2 * 2 - 8 ) + 15 > 9 + 9");
    let operator_two = tree_generator::parse_expression(&mut token_two);
    println!("{:?}",operator_two);
    // tokenization::parse_expression(&mut token_one);
    // print!("\n");
    //************************************************
    //let mut tokens = tokenization::tokenization("( 2 *2 - 8 ) + 15");
    //print!("\n");
    // let operator = tokenization::parse_expression(&mut tokens);
    // print!("\n");
    // let result = logical_evaluations::operation_arithmeticals(&operator);
    // print!("Final Result : {}",result);
}
