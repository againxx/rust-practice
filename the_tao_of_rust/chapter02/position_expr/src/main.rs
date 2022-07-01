fn math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn main() {
    let a = 3;
    let b = 2;
    assert_eq!(math(|| a + b), 5);
    assert_eq!(math(|| a * b), 6);
}
