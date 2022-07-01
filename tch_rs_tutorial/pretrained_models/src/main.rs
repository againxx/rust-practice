use tch::{vision, nn::ModuleT};
use std::env;
use anyhow::Result;

fn main() ->Result<()> {
    let image = vision::imagenet::load_image_and_resize(env::args().nth(1).unwrap(), 224, 224)?;
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);

    let resnet18 = vision::resnet::resnet18(&vs.root(), vision::imagenet::CLASS_COUNT);
    vs.load("./resnet18.ot")?;

    let output = resnet18.forward_t(&image.unsqueeze(0), false).softmax(-1, tch::kind::Kind::Float);

    for (probability, class) in vision::imagenet::top(&output, 5) {
        println!("{:50} {:5.2}%", class, 100. * probability);
    }

    Ok(())
}
