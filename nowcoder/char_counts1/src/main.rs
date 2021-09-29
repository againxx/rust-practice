use std::io;
use std::collections::HashMap;

fn main() {
    let mut input_string = String::new();
    let cin = io::stdin();
    cin.read_line(&mut input_string).unwrap();
    let mut char_count = HashMap::new();
    for c in input_string.chars() {
        let count = char_count.entry(c.to_lowercase().next()).or_insert(0);
        *count += 1;
    }
    input_string.clear();
    cin.read_line(&mut input_string).unwrap();
    let query_char = input_string.chars().next().unwrap().to_lowercase().next();
    let query_count = char_count.get(&query_char).unwrap_or(&0);
    println!("{}", query_count);
}
