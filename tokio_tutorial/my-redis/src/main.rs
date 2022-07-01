use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::create("foo.txt").await?;

    let n = f.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'.", n);

    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();
    // let mut buffer = [0; 10];
    let n = f.read_to_end(&mut buffer).await?;

    println!("Read {} bytes", n);
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
