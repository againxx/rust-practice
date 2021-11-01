struct Solution;

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n = n as usize;
        [&s[n..], &s[..n]].concat()
    }
}

fn main() {
    assert_eq!(Solution::reverse_left_words(String::from("abcdefg"), 2), String::from("cdefgab"));
}
