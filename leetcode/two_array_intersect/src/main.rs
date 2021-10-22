struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let mut result = Vec::new();
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut index2 = 0;
        for num in nums1 {
            while num > nums2[index2] && index2 < nums2.len() - 1 {
                index2 += 1;
            }
            if num == nums2[index2] {
                result.push(num);
                index2 += 1;
            }
            if index2 == nums2.len() {
                break;
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]));
}
