use euler::common;

pub fn execute() {
    print!("Problem 3: ");
    println!("{}", max_prime_factor(600851475143))
}

fn max_prime_factor(number: i64) -> i64 {
    let mut result = 1;
    let mut max = number / 2;
    let mut current = 2;
    while current < max {
        if common::is_multiple(number, current){ 
            if common::is_prime(number / current){
                result = number / current;
                break;
            }
            if common::is_prime(current){
                result = current;
            }
        }
        current += 1;
        max = number / current;
    }
    result
}


