/* use std::env;
use std::fs::File; */
// use std::io::{self, BufRead, Write};

/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

fn simpleArraySum(ar: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..ar.len() {
        println!("{}", ar[i]);
        sum += ar[i];
    }
    return sum;
}

fn main() {
    let ar: Vec<i32> = [1,2,3].to_vec();

    let result = simpleArraySum(&ar);

    println!("result: {}", result);
}
