use euler::common;

pub fn execute() {
    println!("Problem 7: ");
    println!("{}", get_the_prime(6));
}

fn get_the_prime(n: i64) -> i64 {
    let mut prime_n: i64 = 1;
    let mut counter: i64 = 2;
    while prime_n < n{
      counter += 1;
      if common::is_prime(counter){
        prime_n += 1;
      }
      print!("Counter: {}. ", counter);
      println!("Prime_N: {}", prime_n)
    }
    counter
}