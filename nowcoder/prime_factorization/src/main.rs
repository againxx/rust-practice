use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut input: u64 = line.trim().parse().unwrap();
    while input % 2 == 0 {
        print!("{} ", 2);
        input /= 2;
    }
    for i in (3..=(input as f64).sqrt() as u64).step_by(2) {
        while input % i == 0 {
            print!("{} ", i);
            input /= i;
        }
    }
    if input != 1 {
        print!("{} ", input);
    }
}
