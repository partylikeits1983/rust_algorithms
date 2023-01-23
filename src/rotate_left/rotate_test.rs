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

    // size of array
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // number of rotations
    let d = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // initializing input array
    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let array_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        arr.push(array_item);
    }

    let result = rotate_left(d, &arr);

    for i in result {
        println!("{}", i);
    }
}

