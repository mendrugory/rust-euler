use euler::common;

pub fn execute() {
    print!("Problem 2: ");
    println!("{}", fib(1, 1, 0, 4000000))
}

fn fib(current: i64, previous: i64, result: i64, max: i64) -> i64 {
    let new_current = current + previous;
    if new_current >= max {
        result
    }else {
        if common::is_multiple(new_current, 2){
            fib(new_current, current, result + new_current, max)
        }
        else{
            fib(new_current, current, result, max)
        }
    }
}


