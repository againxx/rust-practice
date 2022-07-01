
fn self_attention(x: tch::Tensor) -> tch::Tensor {
    // We assume we have some tensor x with size (b, t, k)
    let raw_weights = x.bmm(&x.transpose(1, 2));
    // apply row-wise softmax
    let weights = raw_weights.softmax(2, tch::Kind::Float);
    weights.bmm(&x)
}

fn main() {
    println!("Hello, world!");
}
