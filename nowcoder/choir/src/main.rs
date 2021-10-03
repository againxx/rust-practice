use std::io::{self, BufRead};

fn compute_left_max_counts(heights: &Vec<u32>) -> Vec<u64> {
    let mut result = vec![0; heights.len()];
    for i in 1..heights.len() {
        for j in 0..i {
            if heights[j] < heights[i] && result[j] >= result[i] {
                result[i] = result[j] + 1;
            }
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let member_num: u64 = line.unwrap().parse().unwrap();
        let mut heights: Vec<u32> = lines.next().unwrap().unwrap().trim().split(' ').map(|h| h.parse().unwrap()).collect();
        let left_higher_counts = compute_left_max_counts(&heights);
        heights.reverse();
        let mut right_higher_counts = compute_left_max_counts(&heights);
        right_higher_counts.reverse();
        let max_reamined_members = left_higher_counts.iter().zip(right_higher_counts.iter()).map(|(x, y)| x + y).max().unwrap() + 1;
        println!("{}", member_num - max_reamined_members);
    }
}
