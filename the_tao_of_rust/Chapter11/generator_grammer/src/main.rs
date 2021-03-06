#![feature(generators, generator_trait)]
use std::ops::Generator;
use std::pin::Pin;

fn main() {
    let mut gen = || {
        yield 1;
        yield 2;
        yield 3;
        return 4;
    };
    for _ in 0..4 {
        let c = Pin::new(&mut gen).resume(());
        println!("{:?}", c);
    }
}
