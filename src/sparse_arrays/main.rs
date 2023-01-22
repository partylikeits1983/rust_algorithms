use std::io::{self, BufRead};
use std::collections::HashMap;

/*
 * Complete the 'matchingStrings' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY stringList
 *  2. STRING_ARRAY queries
 */



fn main() {
    // IO
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // string list count
    let string_list_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // initialize vector of strings
    let mut string_list: Vec<String> = Vec::with_capacity(string_list_count as usize);

    // read in strings
    for _ in 0..string_list_count {
        let string_list_item = stdin_iterator.next().unwrap().unwrap();
        string_list.push(string_list_item);
    }

    // init hashmap
    let mut map = HashMap::new();

    // write string frequency to hashmap
    for string_list_item in string_list {
        if map.contains_key(&string_list_item) {
            let value = map[&string_list_item] + 1;
    
            map.insert(string_list_item, value);
        } else {
            map.insert(string_list_item, 1);
        }
    }

    // queries count
    let queries_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);
    
    // read in queries
    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    // init return vector
    let mut res: Vec<i32> = Vec::with_capacity(queries_count as usize);
    
    // call hashmap and save values to res vector
    for i in queries {
        if map.contains_key(&i) {
            let value = map[&i];
            res.push(value);
        } else {
            res.push(0);
        }
    }

    // print result
    for i in res {
        println!("{}", i); 
    }
}

