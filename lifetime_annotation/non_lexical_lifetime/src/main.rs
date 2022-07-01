use std::collections::HashMap;

fn do_something(_v: &mut &str) {
    todo!()
}

fn main() {
    let mut map = HashMap::new();
    map.insert("hello", "world");
    let key = "hello1";

    match map.get_mut(key) {
        Some(v) => do_something(v),
        None => {
            map.insert(key, "tyr");
        }
    }
}
