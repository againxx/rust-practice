use std::io;
use std::mem;

fn main() {
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    let int_num: i32 = inputs.trim().parse().unwrap();
    let mut count = 0;
    let mut uint_repr: u32 = unsafe {
        mem::transmute(int_num)
    };
    while uint_repr != 0 {
        count += uint_repr & 1;
        uint_repr >>= 1;
    }
    println!("{}", count);
}
