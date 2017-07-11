pub fn execute() {
    print!("Problem 1: ");
    let mut result = 0;
    for i in 0..999 {
        if is_multiple(i, 3) || is_multiple(i, 5) {
            result += i;
        }
    }
    println!("{}", result)
}

fn is_multiple(num: i64, div: i64) -> bool {
    num % div == 0
}
