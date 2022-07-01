use anyhow::Result;
use tch::{data::TextData, nn::{LSTM, Linear, self}, Device};

const LEARNING_RATE: f64 = 0.01;
const HIDDEN_SIZE: i64 = 256;
const SEQ_LEN: i64 = 180;
const BATCH_SIZE: i64 = 256;
const EPOCHS: i64 = 100;
const SAMPLING_LEN: i64 = 1024;

fn sample(data: &TextData, lstm: &LSTM, liner: &Linear, device: Device) -> String {

}

fn main() -> Result<()> {
    let device = Device::cuda_if_available();
    let vs = nn::VarStore::new(device);
    let data = TextData::new("data/input.txt")?;
}
