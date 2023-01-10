use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) -> (i32, i32) {
    let min = arr.iter().min().unwrap();
    let max = arr.iter().max().unwrap();

    let mut sum_max: i32 = 0;
    let mut sum_min: i32 = 0;

    for i in 0..5 {
        if arr[i] == *min {
            // pass
        } else {
            sum_max += arr[i];
        }

        if arr[i] == *max {
            // pass 
        } else {
            sum_min += arr[i];
        }
    }

    return (sum_min, sum_max);

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let (sum_min, sum_max) = mini_max_sum(&arr);

    println!("{} {}", sum_min, sum_max);
}
