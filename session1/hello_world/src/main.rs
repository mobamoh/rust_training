use std::io;

fn main() {
    
    println!("Hello there!");
    println!("What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error reading from stdin");
    println!("Nice to meet you {}",name.trim());

}
