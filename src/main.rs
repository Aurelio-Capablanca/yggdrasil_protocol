mod core;
mod structure;

use std::io::Read;

use crate::{
    core::{
        general_operator::{self},
        tokenization::{self},
        tree_generator::{self},
    },
    structure::domain::Domain,
};

fn main() {
    // let mut one_e = tokenization::tokenization("(34543 +23433) + 89794 * (6745 + 666) / 2", 1_u32);
    // let tree_one = tree_generator::parse_expression(&mut one_e);
    // let domain_settings = Domain::new();
    // let result_one = general_operator::do_maths(&tree_one, &domain_settings);
    // println!("1. = {:?}", result_one);

    /* when A=1, B=0 y C=1 */
    // let mut one_e = tokenization::tokenization("1 && !0 || 1 && 0 || 0 && !1 || 0 && !1", 1_u32);
    // let tree_one = tree_generator::parse_expression(&mut one_e);
    // let domain_settings = Domain::new();
    // let result_one = general_operator::do_maths(&tree_one, &domain_settings);
    // println!("1. = {:?}", result_one);

    let mut flag = true;
    let mut input = String::new();
    while flag {
        std::io::stdin().read_line(&mut input).unwrap_or(0_usize);
        let mut one_e =
            tokenization::tokenization(&input, 1_u32);
        let tree_one = tree_generator::parse_expression(&mut one_e);
        let domain_settings = Domain::new();
        let result_one = general_operator::do_maths(&tree_one, &domain_settings);
        println!("1. = {:?}", result_one);

        println!("do you want to continue? y/n");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("n") {
            flag = false;
        } 
    }

    // let mut one_e = tokenization::tokenization("!(!(1 && 0) || (0 && 1) || (!1 || !0))", 1_u32);
    // let tree_one = tree_generator::parse_expression(&mut one_e);
    // let domain_settings = Domain::new();
    // let result_one = general_operator::do_maths(&tree_one, &domain_settings);
    // println!("1. = {:?}", result_one);

    //Decimal to Octal
    // let mut one_e = tokenization::tokenization("32059.25 ' 10 -> 8", 1_u32);
    // let tree_one = tree_generator::parse_expression(&mut one_e);
    // let result_one = general_operator::do_maths(&tree_one);
    // println!("1. = {:?}", result_one);

    // let mut two_e = tokenization::tokenization("834765.645 ' 10 -> 8", 1_u32);
    // let tree_two = tree_generator::parse_expression(&mut two_e);
    // let domain_settings = Domain::new();
    // let result_two = general_operator::do_maths(&tree_two, &domain_settings);
    // println!("2. = {:?}", result_two);

    // //Octal to Decimal
    // let mut three_e = tokenization::tokenization("34727.36 ' 8 -> 10", 1_u32);
    // let tree_three = tree_generator::parse_expression(&mut three_e);
    // let result_three = general_operator::do_maths(&tree_three);
    // println!("3. = {:?}", result_three);

    // let mut four_e = tokenization::tokenization("24787645.655 ' 8 -> 10", 1_u32);
    // let tree_four = tree_generator::parse_expression(&mut four_e);
    // let result_four = general_operator::do_maths(&tree_four);
    // println!("4. = {:?}", result_four);

    // //Binary to Octal
    // let mut five_e = tokenization::tokenization("111111000.10010 ' 2 -> 8", 1_u32);
    // let tree_five = tree_generator::parse_expression(&mut five_e);
    // let result_five = general_operator::do_maths(&tree_five);
    // println!("5. = {:?}", result_five);

    // let mut six_e = tokenization::tokenization("10110010100101.001101 ' 2 -> 8", 1_u32);
    // let tree_six = tree_generator::parse_expression(&mut six_e);
    // let result_six = general_operator::do_maths(&tree_six);
    // println!("6. = {:?}", result_six);

    // //Octal to Binary
    // let mut seven_e = tokenization::tokenization("5064.50 ' 8 -> 2", 1_u32);
    // let tree_seven = tree_generator::parse_expression(&mut seven_e);
    // let result_seven = general_operator::do_maths(&tree_seven);
    // println!("7. = {:?}", result_seven);

    // let mut eight_e = tokenization::tokenization("3764.121 ' 8 -> 2", 1_u32);
    // let tree_eight = tree_generator::parse_expression(&mut eight_e);
    // let result_eight = general_operator::do_maths(&tree_eight);
    // println!("8. = {:?}", result_eight);

    // // Decimal to Hexadecimal
    // let mut ten_e = tokenization::tokenization("1405.28 ' 10 -> 16", 1_u32);
    // let tree_ten = tree_generator::parse_expression(&mut ten_e);
    // let result_ten = general_operator::do_maths(&tree_ten);
    // println!("9. = {:?}", result_ten);

    // let mut nine_e = tokenization::tokenization("61476 ' 10 -> 16", 1_u32);
    // let tree_nine = tree_generator::parse_expression(&mut nine_e);
    // let result_nine = general_operator::do_maths(&tree_nine);
    // println!("10. = {:?}", result_nine);

    // //Hex to Decimal
    // let mut eleven_e = tokenization::tokenization("defa.1ae ' 16 -> 10", 1_u32);
    // let tree_eleven = tree_generator::parse_expression(&mut eleven_e);
    // let result_eleven = general_operator::do_maths(&tree_eleven);
    // println!("11. = {:?}", result_eleven);

    // let mut twelve_e = tokenization::tokenization("bea210.c29 ' 16 -> 10", 1_u32);
    // let tree_twelve = tree_generator::parse_expression(&mut twelve_e);
    // let result_twelve = general_operator::do_maths(&tree_twelve);
    // println!("12. = {:?}", result_twelve);

    //Hex to Binary

    //Binary to Hex

    /************************ */
    //Sums and Subtracts

    // let mut twenty_one_e = tokenization::tokenization(" 111111000111000111.11111111 ' 2 + 11111000100101010.00001+111111.1101 ' 2 + 1010101010000111111.1110010 ' 2 ", 2_u32);
    // let tree_twenty_one = tree_generator::parse_expression(&mut twenty_one_e);
    // let result_twenty_one = general_operator::do_maths(&tree_twenty_one);
    // println!("21. = {:?}", result_twenty_one);

    // let mut twenty_two_e = tokenization::tokenization("101111101111011111.001 ' 2  + 100000011110010101.101010110 ' 2 + 11111001100010001110001111.11110011 ' 2 + 111100111.10110 ' 2", 2_u32);
    // let tree_twenty_two = tree_generator::parse_expression(&mut twenty_two_e);
    // let result_twenty_two = general_operator::do_maths(&tree_twenty_two);
    // println!("22. = {:?}", result_twenty_two);

    // let mut twenty_three_e = tokenization::tokenization("1111000111001110001111000111000.111 ' 2 - 1111111.11111111 ' 2", 2_u32);
    // let tree_twenty_three = tree_generator::parse_expression(&mut twenty_three_e);
    // let result_twenty_three = general_operator::do_maths(&tree_twenty_three);
    // println!("23. = {:?}", result_twenty_three);

    // let mut twenty_four_e = tokenization::tokenization("111011.1010 ' 2 + 1010101010.11 ' 2 + 11100111001.11 ' 2 - 110010011000111.00001 ' 2", 2_u32);
    // let tree_twenty_four = tree_generator::parse_expression(&mut twenty_four_e);
    // let result_twenty_four = general_operator::do_maths(&tree_twenty_four);
    // println!("24. = {:?}", result_twenty_four);

    /*************************************************************************************************************** */

    // println!(" **** \n 5. \n"); //exercise 5
    // let mut excercise_five = tokenization::tokenization(
    //     "11010101.11 ' 2 + 1110011.10 ' 2 - 11111000.11 ' 2 - 11111111111.0111 ' 2",
    //     2_u32,
    // );
    // let expression_excercise_five = tree_generator::parse_expression(&mut excercise_five);
    // println!("\n{:?}", expression_excercise_five);
    // let result_excercise_five = general_operator::do_maths(&expression_excercise_five);
    // print!("{:?}", result_excercise_five);
}
