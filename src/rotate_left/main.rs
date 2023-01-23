use std::io::{self, BufRead};

/*
 * Complete the 'rotateLeft' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER d
 *  2. INTEGER_ARRAY arr
 */

fn rotateLeft(d: i32, arr: &[i32]) {
    println!("{}", d);
    println!("{}", arr[0]);



}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    println!("n: {}", n);

    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let array_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        arr.push(array_item);
    }

    println!("printing array");

    for i in &arr {
        println!("{}", i);
    }

    println!("passing arr to func");

    rotateLeft(n, &arr);
}
