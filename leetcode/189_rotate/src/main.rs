struct Solution;

impl Solution {
    fn find_desired_index(current_index: usize, k: i32, length: usize) -> usize {
        (current_index + k as usize) % length
    }

    fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        for current_index in 1..length {
            let mut desired_index = Solution::find_desired_index(current_index, k, length);
            while desired_index < current_index {
                desired_index = Solution::find_desired_index(desired_index, k, length);
            }
            if desired_index == current_index {
                desired_index = Solution::find_desired_index(current_index, k, length);
                while desired_index != current_index {
                    nums.swap(current_index, desired_index);
                    desired_index = Solution::find_desired_index(desired_index, k, length)
                }
            }
        }
    }
}

fn main() {
    let mut inputs: Vec<i32> = (1..8).collect();
    Solution::rotate(&mut inputs, 3);
    println!("{:?}", inputs);
}
