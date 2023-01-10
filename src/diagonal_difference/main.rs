use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr[0].len();

    let mut d1 = 0;
    let mut d2 = 0;
    
    let mut j = n;

    for i in 0..n {
        j -= 1;

        d1 += arr[i][i];

        d2 += arr[i][j];        
    }

    let result = (d1 - d2).abs();

    return result;
}

fn main() {
    let arr: Vec<Vec<i32>> = [[11,2,4].to_vec(),[4,5,6].to_vec(),[10,8,-12].to_vec()].to_vec();

    let result = diagonalDifference(&arr);

    println!("result: {}", result);
}
