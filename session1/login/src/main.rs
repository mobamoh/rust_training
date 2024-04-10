use std::io;

use authentication::{greet, login};

fn main() {
    let mut tries = 0;
    println!("🦀🦀🦀🦀🦀🦀🦀 🔐🔐 🦀🦀🦀🦀🦀🦀🦀"); 
    loop {
        println!("Enter your username: ");
        let username = read_input();
        println!("Enter your password: ");
        let password = read_input();

        if login(&username, &password) {
            println!("{}", greet(&username));
            break;
        } else {
            println!("Username or Password incorrect! 🤷‍♂️");
            tries += 1;
            if tries >3 {
                println!("Your account is locked, please contact the admin!");
                break;
            }
        }
    }
}


fn read_input() -> String{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Error reading stdin!");
    buf.trim().to_string()
}