struct Solution;

impl Solution {
    pub fn move_zeroes1(nums: &mut Vec<i32>) {
        for i in (0..nums.len() - 1).rev() {
            if nums[i] == 0 {
                let mut j = i;
                while j + 1 < nums.len() && nums[j + 1] != 0 {
                    nums.swap(j, j + 1);
                    j += 1;
                }
            }
        }
    }

    pub fn move_zeroes2(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                j = (i + 1).max(j);
                while j < nums.len() && nums[j] == 0 {
                    j += 1;
                }
                if j == nums.len() {
                    break;
                } else {
                    nums.swap(i, j);
                }
            }
        }
    }

    pub fn move_zeroes3(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                if i != j {
                    nums[i] = 0;
                }
                j += 1;
            }
        }
    }

}

fn main() {
    let mut inputs = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes3(&mut inputs);
    println!("{:?}", inputs);
}
