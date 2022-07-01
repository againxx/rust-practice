#![allow(dead_code)]

use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");

    let x = foo();
    async {
        read_to_string("file").await;
    };
}

async fn foo() -> usize {
    0
}
