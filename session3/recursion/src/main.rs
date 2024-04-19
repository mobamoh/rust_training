use std::{future::Future, pin::Pin};

use async_recursion::*;

#[tokio::main]
async fn main() {
    let num = 10;
    let fib_num = fibonacci(num);
    println!("fibonacci of {num} is {}", fib_num.await);

    let mut hello = async {
        println!("Hello");
    };

    tokio::pin!(hello);
    (&mut hello).await;
}

#[async_recursion] // makes it easy to work with Pin Box pointer
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}

async fn one() {
    println!("One")
}

async fn two() {
    println!("Two")
}

async fn one_or_two(num: u8) -> Pin<Box<dyn Future<Output = ()>>> {
    match num {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("invalid choice"),
    }
}
