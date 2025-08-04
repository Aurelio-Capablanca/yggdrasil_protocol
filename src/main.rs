mod structure;
mod core;

use crate::core::{logical_evaluations,  tokenization};


fn main() {
    println!("Hello, world!");
    //logical_evaluations::test();
    
    print!("\n");
    let mut token_one = tokenization::tokenization("( 2 *2 - 8 ) + 15 && 9 + 9");
    print!("\n");
    // tokenization::parse_expression(&mut token_one);
    // print!("\n");
    //************************************************
    let mut tokens = tokenization::tokenization("( 2 *2 - 8 ) + 15");
    //print!("\n");
    // let operator = tokenization::parse_expression(&mut tokens);
    // print!("\n");
    // let result = logical_evaluations::operation_arithmeticals(&operator);
    // print!("Final Result : {}",result);
}
