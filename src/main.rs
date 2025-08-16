mod structure;
mod core;

use crate::core::{logical_evaluations,  tokenization, tree_generator};


fn main() {

    // let mut token_one = tokenization::tokenization("( 2 * 2 - 8 ) + 15 && 9 + 9");
    // let operator = tree_generator::parse_expression(&mut token_one);
    // println!("{:?}",operator);
    //
    // let mut token_two = tokenization::tokenization("( 2 * 2 - 8 ) + 15 > 9 + 9");
    // let operator_two = tree_generator::parse_expression(&mut token_two);
    // println!("{:?}",operator_two);
    //
    //


    //(729 *2)+(243 *2)+(81 *2)+(27*1)+(9*1)+(3*2)+(1*2)
    let mut token_start = tokenization::tokenization("(729 *2)+(243 *2)+(81 *2)+(27*1)+(9*1)+(3*2)+(1*2)");
    let expression_getter = tree_generator::parse_expression(&mut token_start);
    println!("{:?}",expression_getter);
    let result = logical_evaluations::arithmetics(&expression_getter);
    println!("{:?}",result);
    println!(" **** ");

    let boolean_only = tokenization::tokenization("T == T");
    // let mut operation_three= tokenization::tokenization("!( ! (75/3>3*25) && (10 - 12<=8) || !(30/3*2-5>20/2*4))");
    // println!();
    // let operator_three = tree_generator::parse_expression(&mut operation_three);
    // println!("{:?}",operator_three);

}
