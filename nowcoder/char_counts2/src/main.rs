use std::io;
use std::collections::HashSet;

fn main() {
    let cin = io::stdin();
    let mut input_buffer = String::new();
    cin.read_line(&mut input_buffer).unwrap();
    let mut char_set = HashSet::new();
    for b in input_buffer.bytes() {
        if b != b'\n' && b <= 127 {
            char_set.insert(b);
        }
    }
    println!("{}", char_set.len());
}
