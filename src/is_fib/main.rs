use std::io;

/* 
// too slow
fn is_square(n: u128) -> bool {
    let mut i = 1;

    while i * i <=n {
        if n % i == 0 && n / i == i {
            return true;
        }
        i += 1;
    }
    return false;
}
*/

fn is_square(n: u128) -> bool {

    let square: u128 = (n as f64).sqrt() as u128;

    if square * square == n {
        return true;
    } else {
        return false;
    }

}

fn is_fibo(n: u128) -> &'static str {
    let mut a: u128 = 5 * n.pow(2) + 4;
    let mut b: u128 = 5 * n.pow(2) - 4;

    if is_square(a) == true || is_square(b) == true {
        return "IsFibo";
    
    } else {
        return "IsNotFibo";
    }
}


fn is_overflow(n: u128) {
    let mut res = 5 * n.pow(2);

    let x = is_square(res);

    println!("{}", res);
    println!("{}", x);
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");

    let t: u128 = input_line.trim().parse().expect("Input not an integer");

    for _ in 0..t {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Failed to read line");
        
        let n: u128 = input_line.trim().parse().expect("Input not an integer");

        // is_overflow(n);
 
        let result = is_fibo(n);

        println!("{}", result);
    }
}
