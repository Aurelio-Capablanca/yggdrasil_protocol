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

    let mut complete_a: Vec<char> = Vec::new();
    if frac_a.is_empty() {
        complete_a = format!("{}{}", int_a, frac_a).chars().collect();
    } else {
        complete_a = format!("{}.{}", int_a, frac_a).chars().collect();
    }

    let mut complete_b: Vec<char> = Vec::new();
    if frac_b.is_empty() {
        complete_b = format!("{}{}", int_b, frac_b).chars().collect();
    } else {
        complete_b = format!("{}.{}", int_b, frac_b).chars().collect();
    }

    (complete_a, complete_b)
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

pub fn sum_binaries(a: &str, b: &str) -> String {
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

fn clean_fraction(value: String) -> (String, String) {
    let set = value.to_string();
    let splitter: Vec<&str> = set.split('.').collect();
    let integer_part: String = splitter.first().unwrap_or(&"0").to_string();
    let fract_part: String = match splitter.get(1) {
        Some(frc) => frc.to_string(),
        None => "".to_string(),
    };
    (integer_part, fract_part)
}

pub fn multiply_binaries(a: String, b: String) -> String {
    let (integer_a, frac_a) = clean_fraction(a);
    let (integer_b, frac_b) = clean_fraction(b);
    let total_space_frac = frac_a.len() + frac_b.len();

    let complete_a = format!("{}{}", integer_a, frac_a);
    let complete_b = format!("{}{}", integer_b, frac_b);
    println!("A : {} ; B : {}", complete_a, complete_b);

    let mut result = "0".to_string();
    for (shift, bit_b) in complete_b.chars().rev().enumerate() {
        if bit_b == '1' {
            let mut partial = complete_a.to_string();
            partial.push_str(&"0".repeat(shift));
            let width = result.len().max(partial.len());
            let partial_padded = format!("{:0>width$}", partial, width = width);
            let result_padded = format!("{:0>width$}", result, width = width);
            result = sum_binaries(&result_padded, &partial_padded);
            println!(
                "Partial: {} + {} = {}",
                result_padded, partial_padded, result
            );
        }
    }

    if total_space_frac > 0 {
        // Pad with leading zeros if needed
        if result.len() <= total_space_frac {
            result = format!("{:0>width$}", result, width = total_space_frac + 1);
        }
        let pos = result.len() - total_space_frac;
        result.insert(pos, '.');
    }
    println!("Result Multiplication : {}", result);
    result
}

pub fn subtract_binaries(a: &str, b: &str) -> String {
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

pub fn divide_binaries(a: &str, b: &str, precision: i32) -> (String, String) {
    let mut quotient = String::new();
    let mut reminder = String::new();

    let mut current_point = String::new();

    for bit in a.to_string().chars() {
        current_point.push(bit);
        if compare_binaries(&current_point, &b.to_string()) >= 0 {
            current_point = subtract_binaries(&current_point, &b.to_string());
            quotient.push('1');
        } else {
            quotient.push('0');
        }
    }

    //reminder = current_point.clone();
    let integer_frac_reminder = current_point.clone();    

    if !current_point.is_empty() && precision > 0_i32 {
        quotient.push('.');
        for _ in 0..precision {
            current_point.push('0');
            if compare_binaries(&current_point, &b.to_string()) >= 0 {
                current_point = subtract_binaries(&current_point, &b.to_string());
                quotient.push('1');
            } else {
                quotient.push('0');
            }
        }
    }

    reminder = current_point.trim_start_matches('0').to_string();
    if reminder.is_empty() {
        reminder = "0".to_string();
    }
    
    quotient = quotient.trim_start_matches('0').to_string();
    if quotient.starts_with('.') {
        quotient.insert(0, '0');
    }
    (quotient, integer_frac_reminder)
}
