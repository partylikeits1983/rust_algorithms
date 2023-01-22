use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'matchingStrings' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY stringList
 *  2. STRING_ARRAY queries
 */

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let stringList_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut stringList: Vec<String> = Vec::with_capacity(stringList_count as usize);

    for _ in 0..stringList_count {
        let stringList_item = stdin_iterator.next().unwrap().unwrap();
        stringList.push(stringList_item);
    }

    for i in stringList {
        println!("{}", i);
    }

}
