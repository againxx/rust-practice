use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let inputs: Vec<i32> = line.unwrap().split(' ').map(|s| s.parse().unwrap()).collect();
        for num in inputs.iter() {
            let used:
        }
    }
}
