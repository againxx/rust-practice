use std::io::{self, BufRead};

fn divide_remains(mut remains: Vec<&i32>, sum1: i32, sum2: i32) -> bool {
    match remains.pop() {
        Some(current_num) => {
            if divide_remains(remains.clone(), sum1 + current_num, sum2) {
                return true;
            }
            if divide_remains(remains.clone(), sum1, sum2 + current_num) {
                return true;
            }
            return false;
        },
        None => sum1 == sum2
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let input_num: u32 = line.unwrap().parse().unwrap();
        let inputs: Vec<i32> = lines.next().unwrap().unwrap().trim().split(' ').map(|s| s.parse().unwrap()).collect();
        let sum1: i32 = inputs.iter().filter(|&x| *x % 5 == 0).sum();
        let sum2: i32 = inputs.iter().filter(|&x| *x % 5 != 0 && *x % 3 == 0).sum();
        let remains: Vec<&i32> = inputs.iter().filter(|&x| *x % 3 != 0 && *x % 5 != 0).collect();
        if divide_remains(remains, sum1, sum2) {
            println!("true");
        } else {
            println!("false");
        }
    }
}
