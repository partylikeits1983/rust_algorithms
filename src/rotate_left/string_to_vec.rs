use std::io::{self, BufRead};

// This is one way to convert a one line string of integers to an i32 vector 

fn read() -> Vec<i32> {
    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for num in &numbers {
        println!("{}", num);
    }

    return numbers;
}

fn main() {
    let arr = read();
    println!("length: {}", arr.len());
}