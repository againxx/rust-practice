use tokio::fs;
use tokio_stream::StreamExt;
use std::{io, collections::HashMap};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();
    let mut reader = csv_async::AsyncReader::from_reader(fs::File::open("/home/ustc1314/.local/share/nvim/cmp-zi/ecdict.csv").await?);
    let mut records = reader.records();
    while let Some(record) = records.next().await {
        let record = record?;
        dict.insert(record.get(0).unwrap().to_string(), vec![
            record.get(1).unwrap_or("").to_string(),
            record.get(2).unwrap_or("").to_string(),
            record.get(3).unwrap_or("").to_string(),
        ]);
    }
    println!("{}", dict.get(&"contains".to_string()).unwrap()[0]);
    println!("{}", dict.get(&"contains".to_string()).unwrap()[1]);
    println!("{}", dict.get(&"contains".to_string()).unwrap()[2]);
    Ok(())
}
