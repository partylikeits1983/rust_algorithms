use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &Vec<i32>) -> Vec<f32> {
    let mut count_p = 0;
    let mut count_n = 0;
    let mut count_z = 0;

    let length = arr.len();

    for i in 0..length {
        if arr[i] > 0 {
            count_p += 1;
        } else if arr[i] == 0 {
            count_z += 1;
        } else {
            count_n += 1;
        }
    }

    let r_p: f32 = (count_p as f32) / (length as f32);
    let r_n: f32 = (count_n as f32) / (length as f32);
    let r_z: f32 = (count_z as f32) / (length as f32);
     
    let mut result: Vec<f32> = Vec::new();
    
    result.push(r_p);
    result.push(r_n);
    result.push(r_z);

    return result;
}

fn main() {
    let arr: Vec<i32> = [-4, 3, -9, 0, 4, 1].to_vec();

    let result = plusMinus(&arr);

    let rp = format!("{:.6}", result[0]);
    let rn = format!("{:.6}", result[1]);
    let rz = format!("{:.6}", result[2]);

    println!("{}", rp);
    println!("{}", rz);
    println!("{}", rn);
}
