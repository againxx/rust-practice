use std::io;

fn main() {
    let cin = std::io::stdin();
    let mut input_buffer = String::new();
    cin.read_line(&mut input_buffer).unwrap();
    let mut inputs: Vec<u32> = input_buffer.trim().split(" ").map(|s| s.parse::<u32>().unwrap()).collect();
    let total_money = inputs[0] / 10;
    let object_num = inputs[1];

    let mut prices: Vec<u32> = Vec::new();
    let mut priorities: Vec<u32> = Vec::new();
    let mut types: Vec<u32> = Vec::new();
    prices.reserve(object_num as usize);
    priorities.reserve(object_num as usize);
    types.reserve(object_num as usize);

    for _ in 0..object_num {
        cin.read_line(&mut input_buffer).unwrap();
        inputs = input_buffer.trim().split(" ").map(|s| s.parse::<u32>().unwrap()).collect();
        prices.push(inputs[0] / 10);
        priorities.push(inputs[1]);
        types.push(inputs[2]);
    }
}
