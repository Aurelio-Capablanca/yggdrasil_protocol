
pub fn parse_strings_to_expression(sentence : &String){
    /* 
    Chop by parenthesis
    
    */
    let chop: Vec<String> = sentence.split('(').collect::<Vec<&str>>()
        .iter().map(|x| x.replace(')', " ")).collect::<Vec<String>>();    
    chop.iter().for_each(|data| print!("Data Got: {}; ",data))
    
}
