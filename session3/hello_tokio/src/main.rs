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
async fn main(){
    hello_tokio().await
}

async fn hello_tokio() {
    println!("Hello Tokio!!")
}