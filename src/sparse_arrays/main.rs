use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

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
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let stringList_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut stringList: Vec<String> = Vec::with_capacity(stringList_count as usize);


    let mut map = HashMap::new();


    for _ in 0..stringList_count {
        let stringList_item = stdin_iterator.next().unwrap().unwrap();


        stringList.push(stringList_item);

        
        
        if map[&stringList_item] >= 1 {
            // println!("true");
    
            let value = map[&stringList_item] + 1;
    
            map.insert(stringList_item, value);
        } else {
            map.insert(stringList_item, 1);
        }

        
    }






    let queries_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    // let res = matchingStrings(&stringList, &queries, &map);

    let mut res: Vec<i32> = Vec::with_capacity(queries_count as usize);


    for i in 0..queries_count {

        let querie = queries.get(i as usize);

        println!("{:?}", querie);

/*         let number = map[&querie];

        res.push(number); */


    }



    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}




