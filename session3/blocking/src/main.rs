use std::time::Duration;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    tokio::join!(
        blocking_task(1, 500),
        blocking_task(2, 1000),
        blocking_task(3, 1500)
    );
}

async fn blocking_task(task: u64, time: u64) {
    println!("Starting task {task}");
    // std::thread::sleep(Duration::from_millis(time)); // this puts the runtime to sleep
    // tokio::time::sleep(Duration::from_millis(time)).await;

    spawn_blocking(move || {
       std::thread::sleep(Duration::from_millis(time)); 
    }); // this will block, EX: to run some sync code 

    println!("Finishing task {task}");
}
