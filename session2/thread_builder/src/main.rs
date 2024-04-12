use std::thread;

fn hello_named_thread() {
    println!("Hello from {} thread!", thread::current().name().unwrap())
}

fn main() {
    println!("Hello, world!");
    hello_named_thread();

    // stack_size reduces the stack size, by default it's 2MB in linux.
    thread::Builder::new()
        .name("bamoh".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(hello_named_thread)
        .unwrap().join().unwrap();
}
