use futures::{executor::block_on, future::join_all, join};
fn main() {
    // say_hello_async().await; // this will not work, it needs an async runtime
    block_on(say_hello_async())
}

async fn say_hello_async() {
    println!("Hello Asynchronisly!!");
    join!(say_hi_async(), say_bye());
    let num_double = double(7).await;
    println!("{num_double}");

    let asyn_vec = vec![double(3), double(6), double(9)];
    let res_vec = join_all(asyn_vec).await;
    println!("{res_vec:?}");
}

async fn say_hi_async() {
    println!("Hi Async!!")
}

async fn say_bye() {
    println!("Goodbye, asynchronisly!!")
}

async fn double(num: u32) -> u32 {
    num * 2
}
