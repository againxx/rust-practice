#![feature(generators, generator_trait)]
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn up_to() -> impl Generator<Yield = u64, Return = ()> {
    || {
        let mut x = 0;
        loop {
            x += 1;
            yield x;
        }
        return ();
    }
}

fn main() {
    let mut gen = up_to();
    for _ in 0..10 {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(i) => println!("{:?}", i),
            _ => println!("Completed"),
        }
    }
}
