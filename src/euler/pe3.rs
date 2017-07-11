pub fn execute() {
    print!("Problem 3: ");
    println!("{}", max_prime_factor(600851475143))
}

fn is_multiple(num: i64, div: i64) -> bool {
    num % div == 0
}

fn is_prime(number: i64) -> bool{
    let mut result = true;
    let mut current = 2;
    let mut max = number / current;
    while current < max{
        if is_multiple(number, current){
            result = false;
            break;
        }
        else {
            current += 1;
            max = number / current;
        }
    } 
    result
}

fn max_prime_factor(number: i64) -> i64 {
    let mut result = 1;
    let mut max = number / 2;
    let mut current = 2;
    while current < max {
        if is_multiple(number, current){ 
            if is_prime(number / current){
                result = number / current;
                break;
            }
            if is_prime(current){
                result = current;
            }
        }
        current += 1;
        max = number / current;
    }
    result
}


