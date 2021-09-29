struct Solution;

impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut current_index = 0;
        let mut search_index = 1;
        while search_index < nums.len() {
            if nums[search_index] != nums[current_index] {
                if nums[current_index] >= nums[current_index + 1] {
                    nums.swap(current_index + 1, search_index);
                }
                current_index += 1;
                search_index += 1;
            } else {
                search_index += 1;
            }
        }
        (current_index + 1) as i32
    }
}

fn main() {
    let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let length = Solution::remove_duplicates(&mut input);
    assert_eq!(length, 5);
    for i in 0..length as usize {
        println!("{}", input[i]);
    }
}
