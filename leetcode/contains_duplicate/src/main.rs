use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut existing_nums: HashSet<i32> = HashSet::new();
        for num in nums {
            if existing_nums.contains(&num) {
                return true;
            }
            existing_nums.insert(num);
        }
        false
    }
}

fn main() {
    println!("{}", Solution::contains_duplicate(vec![1, 2, 1, 3, 4]));
}
