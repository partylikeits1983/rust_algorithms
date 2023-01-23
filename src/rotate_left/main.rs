use std::io::{self, BufRead};

/*
* Complete the 'rotateLeft' function below.
*
* The function is expected to return an INTEGER_ARRAY.
* The function accepts following parameters:
*  1. INTEGER d
*  2. INTEGER_ARRAY arr
*/

/* 
sample input:
5 4
1 2 3 4 5

sample output:
5 1 2 3 4
*/

fn rotate_left(d: i32, arr: &[i32]) -> Vec<i32> {
    let mut i = d;

    let mut rotate_arr: Vec<i32> = Vec::with_capacity(d as usize);

    /* 
    d: 2
    arr: 1 2 3 4 5

    rotate_arr: 3 4 5
    */

    while i < arr.len() as i32 {
        let item = arr[i as usize];
        rotate_arr.push(item);

        i += 1;
    }

    /* 
    d: 2
    i: 0
    arr: 1 2 3 4 5
         x x
    
    rotate_arr: 3 4 5 1 2
    */

    i = 0;
    while i < d {
        let item = arr[i as usize];
        rotate_arr.push(item);

        i += 1;
    }

    return rotate_arr;
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    // let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let d = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();


    let result = rotate_left(d, &arr);

    let result_string: String = result.into_iter().map(|i| i.to_string() + " ").collect::<String>();

    println!("{}", result_string);
}

