struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        for i in (0..digits.len()).rev() {
            if carry == 0 {
                break;
            }
            let added_num = digits[i] + carry;
            digits[i] = added_num % 10;
            carry = added_num / 10;
        }
        if carry != 0 {
            digits.insert(0, carry);
        }
        digits
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![9, 9, 9, 9]));
}
