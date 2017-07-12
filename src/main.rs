mod euler;

use std::process::Command;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::{thread, time};

fn main() {
    let functions = get_functions();
    let mut executing = true;

    clean_screen();

    println!("*** Project Euler ***\n");
    
    
    while executing {
        println!("Which problem would you like to solve? ");
        let input = read_input();
        if input < 0{
            println!("Wrong input !!! The input has to be an integer.");
        }else if input == 0{
            executing = false;
            println!("Bye !!");
        }else{
            match functions.get(&input) {
                Some(f) => f(),
                None => println!("The function {} has not been implmented yet", input)
            }
        }
        println!(" ")
    }
}

fn get_functions() -> HashMap<i32, fn()>{
    let mut functions: HashMap<i32, fn()> = HashMap::new();

    functions.insert(1, euler::pe1::execute);
    functions.insert(2, euler::pe2::execute);
    functions.insert(3, euler::pe3::execute);
    functions.insert(4, euler::pe4::execute);
    functions.insert(5, euler::pe5::execute);
    functions.insert(6, euler::pe6::execute);

    functions
}


fn read_input() -> i32 {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut buffer).expect("Could not read line");
    match buffer.trim(){
        "quit" => 0,
        "none" => 0,
        "exit" => 0,
        _ => match buffer.trim().parse::<i32>() {
                Ok(n) => n,
                Err(_err) => -1
            }
    }
}

fn clean_screen(){
    let cmd = Command::new("clear").spawn().expect("failed to clean the screen"); 
    let ten_millis = time::Duration::from_millis(10);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
}
