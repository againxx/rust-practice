use std::io::{self, BufRead};

fn depth_first_search(result: f32, inputs: &Vec<f32>, mut used: Vec<bool>) -> bool {
    if result == 24.0 && used.iter().all(|&x| x) {
        return true;
    }
    for i in 0..inputs.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        if depth_first_search(result + inputs[i], inputs, used.clone()) {
            return true;
        }
        if depth_first_search(result - inputs[i], inputs, used.clone()) {
            return true;
        }
        if depth_first_search(result * inputs[i], inputs, used.clone()) {
            return true;
        }
        if depth_first_search(result / inputs[i], inputs, used.clone()) {
            return true;
        }
        used[i] = false;
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let inputs: Vec<f32> = line.unwrap().split(' ').map(|s| s.parse().unwrap()).collect();
        let mut success = false;
        for i in 0..inputs.len() {
            let mut used = vec![false; 4];
            used[i] = true;
            success |= depth_first_search(inputs[i], &inputs, used);
        }
        if success {
            println!("true");
        } else {
            println!("false");
        }
    }
}
