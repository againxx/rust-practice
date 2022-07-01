use tokio::sync::mpsc::{self, Receiver};

async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    Some(format!("Received number {}", i))
}

#[tokio::main]
async fn main() {
    let (tx, mut rx): (_, Receiver<i32>) = mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        };
    }
}
