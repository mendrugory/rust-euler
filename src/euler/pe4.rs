pub fn execute() {
    print!("Problem 4: ");
    println!("{}", max_palidrome())
}

fn is_palindrome(number: i64) -> bool{
    let number_str: String = number.to_string();
    let mut result = true;
    for (i, c) in number_str.chars().enumerate(){
        if c != number_str.chars().nth(number_str.len() - i - 1).unwrap(){
            result = false;
            break;
        }
    }
    result
}

fn max_palidrome() -> i64 {
    let mut result = 1;
    let mut v: Vec<i64> = Vec::new();
    for i in 100..1000{
        for j in 100..1000{
            v.push(i * j);
        }
    }
    v.sort_by(|a, b| b.cmp(a));
    for num in v{
        if is_palindrome(num){
            result = num;
            break;
        }
    }
    result
}


