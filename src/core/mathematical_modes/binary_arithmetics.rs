
fn normalize_binaries(a: &str, b: &str) -> (Vec<char>, Vec<char>) {
    let split_a: Vec<&str> = a.split('.').collect();
    let split_b: Vec<&str> = b.split('.').collect();
    let (mut int_a, mut frac_a) = (
        split_a[0].to_string(),
        split_a.get(1).unwrap_or(&"").to_string(),
    );
    let (mut int_b, mut frac_b) = (
        split_b[0].to_string(),
        split_b.get(1).unwrap_or(&"").to_string(),
    );
    let max_int = int_a.len().max(int_b.len());
    int_a = format!("{:0>width$}", int_a, width = max_int);
    int_b = format!("{:0>width$}", int_b, width = max_int);

    let max_frac = frac_a.len().max(frac_b.len());
    frac_a = format!("{:0<width$}", frac_a, width = max_frac);
    frac_b = format!("{:0<width$}", frac_b, width = max_frac);

    (
        format!("{}.{}", int_a, frac_a).chars().collect(),
        format!("{}.{}", int_b, frac_b).chars().collect(),
    )
}

fn compare_binaries(a: &str, b: &str) -> i32 {
    let a_part = a.trim_start_matches('0');
    let b_part = b.trim_start_matches('0');

    if a_part.len() > b_part.len() {
        return 1;
    } else if a_part.len() < b_part.len() {
        return -1;
    } else {
        for (ac, bc) in a_part.chars().zip(b_part.chars()) {
            if ac > bc {
                return 1;
            }
            if ac < bc {
                return -1;
            }
        }
        return 0;
    }
}

pub fn sum_binaries(a: f64, b: f64) -> String {
    let (a_part, b_part) = normalize_binaries(&a.to_string(), &b.to_string());
    println!("{:?} + {:?}", a_part, b_part);
    let mut carry = 0;
    let mut result: Vec<char> = Vec::new();
    for i in (0..a_part.len()).rev() {
        if *a_part.get(i).unwrap_or(&'_') == '.' {
            result.push('.');
            continue;
        }
        let bit_a = a_part.get(i).unwrap_or(&'0').to_digit(2).unwrap_or(0);
        let bit_b = b_part.get(i).unwrap_or(&'0').to_digit(2).unwrap_or(0);
        let b_sum = bit_a + bit_b + carry;
        result.push(std::char::from_digit(b_sum % 2, 2).unwrap_or('0'));
        carry = b_sum / 2;
    }
    if carry > 0 {
        result.push('1');
    }
    result.into_iter().rev().collect()
}

pub fn subtract_binaries(a: f64, b: f64) -> String {
    let (a_part, b_part) = normalize_binaries(&a.to_string(), &b.to_string());
    let mut borrow = 0;
    let mut result: Vec<char> = Vec::new();

    for i in (0..a_part.len()).rev() {
        if *a_part.get(i).unwrap_or(&'_') == '.' {
            result.push('.');
            continue;
        }
        let bit_a = a_part.get(i).unwrap_or(&'0').to_digit(2).unwrap_or(0);
        let bit_b = b_part.get(i).unwrap_or(&'0').to_digit(2).unwrap_or(0);
        let mut diference = bit_a as i32 - bit_b as i32 - borrow as i32;
        if diference < 0_i32 {
            diference += 2;
            borrow = 1;
        } else {
            borrow = 0;
        }
        result.push(char::from_digit(diference as u32, 2).unwrap_or('0'));
    }
    result.into_iter().rev().collect()
}


fn clean_fraction(value : f64)-> (i128, i64){
    let set = value.to_string();
    let splitter :Vec<&str> = set.split('.').collect();
    let integer_part : i128 = i128::from_str_radix(splitter.first().unwrap_or(&"0"), 2).unwrap_or(0_i128);
    let fract_part: i64 = match  splitter.get(1) {
        Some(frc) => frc.parse::<i64>().unwrap_or(0_i64),
        None => 0_i64
    }; 
    (integer_part, fract_part)
}

pub fn muiltiply_binaries(a: f64, b: f64) -> String {
    let (integer_a, frac_a) = clean_fraction(a);
    let (integer_b, frac_b) = clean_fraction(b);
    let total_space_frac = frac_a as usize + frac_b as usize;

    let mut product = 0;
    for (shift, bit_b) in integer_b.to_string().chars().rev().enumerate() {
        product = bit_b.to_digit(2).unwrap() as i128 * integer_a;
        println!("Product : {:?}; total Space : {:?}",product,total_space_frac);
    }
    let mut bin = format!("{:?}",product);

    if total_space_frac > 0 && bin.len()  > total_space_frac {
        bin.insert(bin.len() - total_space_frac , '.');        
    } else if total_space_frac > 0 {
        let pin = "0.".to_string() + &"0".repeat(total_space_frac - bin.len()) + &bin;
        return pin;
    }

    // for (shift, bit_b) in b_member.chars().rev().enumerate() {
    //     if bit_b == '1' {
    //         let partial = &mut a_member;
    //         partial.push_str(&"0".repeat(shift));
    //         result = sum_binaries(
    //             result.parse::<f64>().unwrap_or(0_f64),
    //             partial.parse::<f64>().unwrap_or(0_f64),
    //         );
    //     }
    // }
    bin
}

pub fn divide_binaries(a: f64, b: f64) -> (String, String) {
    let mut quotient = String::new();
    let mut reminder = String::new();

    let mut current_point = String::new();
    let a_frac: String = a
        .fract()
        .to_string()
        .strip_prefix("0.")
        .unwrap_or(&"0")
        .chars()
        .collect();
    let b_frac: String = b
        .fract()
        .to_string()
        .strip_prefix("0.")
        .unwrap_or(&"0")
        .chars()
        .collect();
    let precision = b_frac.len() as i32 + a_frac.len() as i32 / 2 as i32;

    for bit in a.to_string().chars() {
        current_point.push(bit);
        if compare_binaries(&current_point, &b.to_string()) >= 0 {
            current_point = subtract_binaries(current_point.parse::<f64>().unwrap_or(0_f64), b);
            quotient.push('1');
        } else {
            quotient.push('0');
        }
    }

    if !current_point.is_empty() && precision > 0_i32{
        quotient.push('.');
        for _ in 0..precision {
            current_point.push('0');
            if compare_binaries(&current_point, &b.to_string()) >= 0 {
                current_point = subtract_binaries(current_point.parse::<f64>().unwrap_or(0_f64), b);
                quotient.push('1');
            } else {
                quotient.push('0');
            }
        }
    }

    reminder = current_point;
    (quotient, reminder)
}
