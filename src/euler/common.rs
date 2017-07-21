use std::string::ToString;


pub fn is_multiple(num: i64, div: i64) -> bool {
    num % div == 0
}

pub fn is_prime(number: i64) -> bool {
    let mut result = true;
    let mut current = 2;
    let mut max = number / current;
    while current <= max {
        if is_multiple(number, current) {
            result = false;
            break;
        } else {
            current += 1;
            max = number / current;
        }
    }
    result
}

pub fn is_palindrome<T: ToString>(number: T) -> bool {
    let number_str: String = number.to_string();
    let mut result = true;
    for (i, c) in number_str.chars().enumerate() {
        if c != number_str.chars().nth(number_str.len() - i - 1).unwrap() {
            result = false;
            break;
        }
    }
    result
}
