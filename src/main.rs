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
    let mut token_start = tokenization::tokenization("((84 / 2) + (15* 3) - (120/4)) * 2 - ((45 * 2) - (36 / 3))");
    let expression_getter = tree_generator::parse_expression(&mut token_start);
    println!("{:?} \n",expression_getter);
    let result = logical_evaluations::mathematics(&expression_getter);
    println!("\n{:?}",result);
    println!(" **** \n");

    // let mut boolean_ex = tokenization::tokenization("!65 == T");
    // let expression_two = tree_generator::parse_expression(&mut boolean_ex);
    // println!("\n{:?}",expression_two);
    // let result_two = logical_evaluations::mathematics(&expression_two);
    // println!("\n{:?}",result_two);
    // println!(" **** \n");

    // let mut operation_three= tokenization::tokenization("!( ! (75/3>3*25) && (10 - 12<=8) || !(30/3*2-5>20/2*4))");
    // println!();
    // let operator_three = tree_generator::parse_expression(&mut operation_three);
    // println!("{:?}",operator_three);

    // let mut boolean_ex2 = tokenization::tokenization("34 -> 10");
    // let expression_two2 = tree_generator::parse_expression(&mut boolean_ex2);
    // println!("\n{:?}",expression_two2);
    // // let result_two = logical_evaluations::mathematics(&expression_two2);
    // // println!("\n{:?}",result_two);
    // println!(" **** \n");
}
