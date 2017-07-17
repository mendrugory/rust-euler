use euler::common;

pub fn execute() {
    print!("Problem 1: ");
    let mut result = 0;
    for i in 3..999 {
        if common::is_multiple(i, 3) || common::is_multiple(i, 5) {
            result += i;
        }
    }
    println!("{}", result)
}
