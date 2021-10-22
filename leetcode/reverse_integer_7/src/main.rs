struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result = 0;
        let mut num;
        while x != 0 {
            num = x % 10;
            x /= 10;
            if (result > std::i32::MAX / 10) || (result == std::i32::MAX / 10 && num > std::i32::MAX % 10 ) {
                result = 0;
                break;
            }
            if (result < std::i32::MIN / 10) || (result == std::i32::MIN / 10 && num < std::i32::MIN % 10) {
                result = 0;
                break;
            }
            result = result * 10 + num;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::reverse(153423646));
}
