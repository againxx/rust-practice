use anyhow::Result;
use tch::nn::{self, Module, OptimizerConfig};
use tch::Device;

const LABELS: i64 = 10;
const BATCH_SIZE: i64 = 128;

fn net(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::conv2d(vs / "conv1", 1, 32, 5, Default::default()))
        .add_fn(|xs| xs.relu())
        .add_fn(|xs| xs.max_pool2d_default(2))
        .add(nn::conv2d(vs / "conv2", 32, 64, 3, Default::default()))
        .add_fn(|xs| xs.relu())
        .add_fn(|xs| xs.max_pool2d_default(2))
        .add_fn(|xs| xs.flat_view())
        .add(nn::linear(vs / "fc1", 1600, 512, Default::default()))
        .add(nn::linear(vs / "fc2", 512, LABELS, Default::default()))
}

fn run() -> Result<()> {
    let mnist_data = tch::vision::mnist::load_dir("data")?;
    let vs = nn::VarStore::new(Device::Cuda(0));
    let net = net(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3)?;
    for epoch in 1..200 {
        let mut average_loss = 0.0;
        for (bimages, blabels) in mnist_data.train_iter(BATCH_SIZE).shuffle().to_device(vs.device()) {
            let loss = net
                .forward(&bimages.reshape(&[-1, 1, 28, 28]))
                .cross_entropy_for_logits(&blabels.to_device(vs.device()));
            opt.backward_step(&loss);
            average_loss += f64::from(loss);
        }
        average_loss /= 60000. / 128.;
        let test_accuracy = net
            .forward(&mnist_data.test_images.reshape(&[-1, 1, 28, 28]).to_device(vs.device()))
            .accuracy_for_logits(&mnist_data.test_labels.to_device(vs.device()));
        println!(
            "epoch: {:4} train loss: {:8.5} test acc: {:5.2}%",
            epoch,
            average_loss,
            100. * f64::from(&test_accuracy)
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    run()?;
    Ok(())
}
