mod core;
mod structure;

use crate::core::{
    general_operator::{self, do_maths},
    tokenization, tree_generator,
};

fn main() {
    
    println!(" **** \n 4. \n"); //exercise 4
    let mut excercise_four =
        tokenization::tokenization("11100001000100 ' 2 -  11101101111 ' 2", 2_u32);
    let expression_excercise_four = tree_generator::parse_expression(&mut excercise_four);
    println!("\n{:?}", expression_excercise_four);
    let result_excercise_four = general_operator::do_maths(&expression_excercise_four);
    print!("{:?}", result_excercise_four);

    println!(" **** \n 5. \n"); //exercise 5 
    let mut excercise_five = tokenization::tokenization(
        "11010101.11 ' 2 + 1110011.10 ' 2 - 11111000.11 ' 2 - 11111111111.0111 ' 2",
        2_u32,
    );
    let expression_excercise_five = tree_generator::parse_expression(&mut excercise_five);
    println!("\n{:?}", expression_excercise_five);
    let result_excercise_five = general_operator::do_maths(&expression_excercise_five);
    print!("{:?}", result_excercise_five);

    println!(" **** \n 6. \n"); //exercise 4
    let mut excercise_six = tokenization::tokenization("1000101111 ' 2 *  1101 ' 2", 2_u32);
    let expression_excercise_six = tree_generator::parse_expression(&mut excercise_six);
    println!("\n{:?}", expression_excercise_six);
    let result_excercise_six = general_operator::do_maths(&expression_excercise_six);
    print!("{:?}", result_excercise_six);

    /* 7 */
    println!(" ****  \n .7 \n");
    let mut excer_seven = tokenization::tokenization("1171477.1344 ' 8 -> 10", 1_u32);
    println!("\n{:?}", excer_seven);
    let mut expression_excer_seven = tree_generator::parse_expression(&mut excer_seven);
    println!("Expression : \n{:?}", expression_excer_seven);
    let result_excer_sever = do_maths(&mut expression_excer_seven);
    print!("{:?}", result_excer_sever);
    //4F33F.2E4
    //324415.18066
    println!(" **** \n");
    let mut excer_seven_h = tokenization::tokenization("324415.18066 ' 10 -> 16", 1_u32);
    println!("\n{:?}", excer_seven_h);
    let mut expression_excer_seven_h = tree_generator::parse_expression(&mut excer_seven_h);
    println!("Expression : \n{:?}", expression_excer_seven_h);
    let result_excer_sever_h = do_maths(&mut expression_excer_seven_h);
    print!("{:?}", result_excer_sever_h);
    /* 7 */

    println!(" **** \n 8 \n"); //exercise 8
    let mut excercise_eight = tokenization::tokenization("110101011 ' 2 /  1001 ' 2", 2_u32);
    let expression_excercise_eight = tree_generator::parse_expression(&mut excercise_eight);
    println!("\n{:?}", expression_excercise_eight);
    let result_excercise_eight = general_operator::do_maths(&expression_excercise_eight);
    print!("{:?}", result_excercise_eight);

    println!(" **** \n 9 \n"); //exercise 9 
    let mut excercise_five = tokenization::tokenization("11010 ' 2 + 11101 ' 2", 2_u32);
    let expression_excercise_five = tree_generator::parse_expression(&mut excercise_five);
    println!("\n{:?}", expression_excercise_five);
    let result_excercise_five = general_operator::do_maths(&expression_excercise_five);
    print!("{:?}", result_excercise_five);

    println!(" **** \n");
    let mut excercise_five_s = tokenization::tokenization("110111 ' 2 + 10101 ' 2", 2_u32);
    let expression_excercise_five_s = tree_generator::parse_expression(&mut excercise_five_s);
    println!("\n{:?}", expression_excercise_five_s);
    let result_excercise_five_s: structure::response::Response =
        general_operator::do_maths(&expression_excercise_five_s);
    print!("{:?}", result_excercise_five_s);
    //exercise 9
   
}
