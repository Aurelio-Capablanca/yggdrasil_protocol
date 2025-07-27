
pub fn parse_strings_to_expression(sentence : &String){
    /* 
    Chop by parenthesis
    
    */
    let chop: Vec<&str> = sentence.split('(').collect();
    chop.iter().for_each(|data| print!("Data Got: {}; ",data))
    
}
