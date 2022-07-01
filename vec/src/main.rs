fn main() {
    let values = vec![1, 32, 10, 51];
    let query = 32;
    let pos = find_value(&values, query);
    println!("{:?}", pos);
    println!("{:?}", values);
}

fn find_value(values: &Vec<i32>, query: i32) -> Option<usize> {
    for (pos, &val) in values.iter().enumerate() {
        if val == query {
            return Some(pos);
        }
    }
    None
}
