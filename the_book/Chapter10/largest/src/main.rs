fn largest<T: PartialOrd>(inputs: &[T]) -> &T {
    let mut largest = &inputs[0];
    for item in inputs.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let i_vec = vec![1, 5, 7, 3, 4, 10];
    println!("{}", largest(&i_vec));

    let c_vec = vec!['a', 'e', 'z', 'd'];
    println!("{}", largest(&c_vec));
}
