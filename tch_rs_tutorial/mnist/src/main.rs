use anyhow::Result;

mod linear;
mod nn;
mod conv;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let model = if args.len() < 2 {None} else {Some(args[1].as_str())};
    match model {
        None => nn::run(),
        Some("linear") => linear::run(),
        Some("nn") => nn::run(),
        Some("conv") => conv::run(),
        Some(_) => nn::run(),
    }
}
