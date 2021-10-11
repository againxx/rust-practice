struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_indexes: Vec<usize> = (0..nums.len()).collect();
        sorted_indexes.sort_by_key(|&i| nums[i]);
        let mut start = 0;
        let mut end = sorted_indexes.len() - 1;
        while start != end {
            let sum = nums[sorted_indexes[start]] + nums[sorted_indexes[end]];
            if sum == target {
                break;
            }
            else if sum < target {
                start += 1;
            }
            else {
                end -= 1;
            }
        }
        vec![sorted_indexes[start] as i32, sorted_indexes[end] as i32]
    }
}

fn main() {
    let nums = vec![3, 2, 3];
    let result = Solution::two_sum(nums, 6);
    println!("{:?}", result);
}
