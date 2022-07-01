fn find_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let s = "Hello world".to_string();
    let word = find_word(&s);
    println!("{}", word);
}
