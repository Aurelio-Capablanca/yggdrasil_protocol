pub fn normalize_binaries(a: &str, b: &str) -> (Vec<char>, Vec<char>) {
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
