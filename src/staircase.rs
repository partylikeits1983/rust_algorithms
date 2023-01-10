use std::io;

fn staircase(n: usize) {
    for i in 1..n+1 {
        let blank_len = n - i;
        println!("{0}{1}", " ".repeat(blank_len), "#".repeat(i));
    }
}

fn main() {
    let mut staircase_len_str = String::new();
    io::stdin().read_line(&mut staircase_len_str).unwrap();

    let n: usize = staircase_len_str.trim().parse().unwrap();

    staircase(n);
}