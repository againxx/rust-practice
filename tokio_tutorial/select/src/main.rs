use std::io;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

pub fn process(_: tokio::net::TcpStream) {}

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send(()).unwrap();
    });

    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        res = async {
            let (socket, _) = listener.accept().await?;
            tokio::spawn(async move { process(socket); });
            Ok::<_, io::Error>(())
        } => { res?; }
        _ = rx => {
            println!("terminating accept loop");
        }
    };
    Ok(())
}
