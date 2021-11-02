struct Solution;

impl Solution {
    pub fn find_repeat_number(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if nums[i] == i as i32 {
                continue;
            }
            let temp = nums[i];
            if nums[i] == nums[temp as usize] {
                return temp;
            } else {
                nums.swap(i, temp as usize);
            }
            if nums[nums[i] as usize] == nums[i] {
                return nums[i];
            }
        }
        -1
    }
}

fn main() {
    assert_eq!(Solution::find_repeat_number(vec![3, 4, 2, 0, 0, 1]), 0);
}
