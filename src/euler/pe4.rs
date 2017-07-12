use euler::common;

pub fn execute() {
    print!("Problem 4: ");
    println!("{}", max_palidrome())
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
        if common::is_palindrome(num){
            result = num;
            break;
        }
    }
    result
}


