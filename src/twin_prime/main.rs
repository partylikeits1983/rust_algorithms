
/*     
#include <stdio.h>  
    #include <conio.h>  
    int main ()  
    {  
        // declare variables  
        int i, num, count = 0;  
        printf (" Enter the last number: ");  
        scanf (" %d", &num); // get the last number  
          
        for (i = 2; i <= num; i++)  
        {  
            if (twinprime (i) && twinprime (i+2))  
            {  
                printf (" \n The twin prime number is: (%d, %d) ", i, i+2);  
                count++; // counter increment by 1  
            }  
        }  
        printf (" \n \n Total number of twin prime pairs: %d", count);  
        return 0;  
    }  
    // function definition   
    int twinprime( int n)  
    {  
        int i = 2;  
        // use for loop to find the twin prime  
        for (i = 2; i <= n/2; i++)  
        {  
            // if n is completely divisible by 1 without leaving any remainder, it returns 0  
            if (n%i == 0)  
                return 0;  
        }  
        // otherwise it returns 1  
        if (i > n / 2)  
            return 1;  
    }   
*/


use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'solve' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER m
 */

fn solve(mut n: i32, m: i32) -> i32 {

    let mut counter = 0;

    while (n <= m) {

        if twin_prime(n) == false && twin_prime(m) == false {
            counter += 1;
        }

        n +=1
    }

    return counter;


}

fn twin_prime(n: i32) -> bool {
    let mut i: i32 = 2;

    while i <= n / 2 {

        if (n % i == 0) {
            return true;
        }

        i += 1;
    }

    if (i > n / 2) {
        return false;
    } else {
        return true;
    }
}



fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = solve(n, m);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}
