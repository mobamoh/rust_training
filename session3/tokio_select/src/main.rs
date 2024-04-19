use std::time::Duration;

use tokio::sync::{broadcast, mpsc};

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = do_task()=> println!("do_task comes first"),
        _ = timeout(0.5) => println!("timeout comes first")
    }

    let (tx, rx) = mpsc::channel::<u32>(1);
    let (broadcast_tx, broadcast_rx) = broadcast::channel::<u32>(1);

    tokio::spawn(receiver(rx, broadcast_rx));

    for count in 0..10 {
        if count % 2 == 0 {
            tx.send(count).await.unwrap();
        } else {
            broadcast_tx.send(count).unwrap();
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn receiver(mut rx: mpsc::Receiver<u32>, mut broadcast_rx: broadcast::Receiver<u32>) {
    loop {
        tokio::select! {
            Some(n) = rx.recv() => println!("Received message {n} on the mpsc channel"),
            Ok(n) = broadcast_rx.recv() => println!("Received message {n} on the broadcast channel"),
        }
    }
}

async fn do_task() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

async fn timeout(secs: f64) {
    tokio::time::sleep(Duration::from_secs_f64(secs)).await;
}
