use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let numbers: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let expected_sum = n * (n + 1) / 2;
    let actual_sum: usize = numbers.iter().sum();

    println!("{}", expected_sum - actual_sum);
}
