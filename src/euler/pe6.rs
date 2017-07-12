use std::thread;

pub fn execute() {
    print!("Problem 6: ");
    println!("{}", diff());
}

fn diff() -> i64 {
    let t1 = square_of_sum(1, 101);
    let t2 = sum_of_squares(1, 101);
    t1.join().unwrap() - t2.join().unwrap()
}


fn sum_of_squares(init: i64, end: i64) -> thread::JoinHandle<i64> {
  let handle = thread::spawn(move || {  
      let mut result = 0;
      for i in init..end {
        result += i * i;
      }
      result
  });
  handle
}

fn square_of_sum(init: i64, end: i64) -> thread::JoinHandle<i64> {
  let handle = thread::spawn(move || {  
      let mut result = 0;
      for i in init..end {
        result += i;
      }
      result * result
  });
  handle
}