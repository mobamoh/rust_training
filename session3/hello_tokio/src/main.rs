// use tokio::runtime;

/* fn main() { // Long form using builder pattern
    let rt = runtime::Builder::new_multi_thread()
    .enable_all()
    .worker_threads(2)
    .build()
    .unwrap();

    rt.block_on(hello_tokio());
} */

#[tokio::main]
async fn main() {
    // hello_tokio().await;
    // ticker().await;

    // let _ = tokio::join!(tokio::spawn(hello_tokio()), tokio::spawn(ticker()));

    // tokio::spawn(ticker());
    // hello_tokio().await;

    let _ =tokio::join!(
        tokio::spawn(hello_tokio()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker())
    );
}

async fn hello_tokio() {
    println!("Hello Tokio!!")
}

async fn ticker() {
    for i in 0..10 {
        println!("tick: {i}");
        tokio::task::yield_now().await;
    }
}
