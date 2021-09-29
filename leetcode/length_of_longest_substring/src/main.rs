use std::collections::HashMap;

// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         let mut max_length = 0;
//         let mut substring = String::new();
//         for c in s.chars() {
//             match substring.find(c) {
//                 Some(index) => {
//                     max_length = max_length.max(substring.chars().count());
//                     substring = substring[index + 1..].to_string();
//                     substring.push(c);
//                 },
//                 None => substring.push(c),
//             }
//         }
//         max_length.max(substring.chars().count()) as i32
//     }
// }

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        let mut substring_begin = 0;
        let mut last_char_indexes = HashMap::new();
        for (index, c) in s.chars().enumerate() {
            let &mut last_duplicate = last_char_indexes.entry(c).or_insert(index);
            if last_duplicate >= substring_begin  && last_duplicate < index {
                max_length = max_length.max(index - substring_begin);
                substring_begin = last_duplicate + 1;
            }
            last_char_indexes.insert(c, index);
        }
        max_length.max(s.len() - substring_begin) as i32
    }
}

fn main() {
    let s = String::from("pwwkew");
    let result = Solution::length_of_longest_substring(s);
    println!("{}", result);
}
