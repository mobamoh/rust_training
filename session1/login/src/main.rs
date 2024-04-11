use std::io;

use authentication::{greet, login, LoginAction, LoginRole};

fn main() {
    let mut tries = 0;
    println!("🦀🦀🦀🦀🦀🦀🦀 🔐🔐 🦀🦀🦀🦀🦀🦀🦀");
    loop {
        println!("Enter your username: ");
        let username = read_input();
        println!("Enter your password: ");
        let password = read_input();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                println!("{}", greet(&username));
                match role {
                    LoginRole::Admin => {
                        println!("You are logged in as an ADMIN!")
                    }
                    LoginRole::User => {
                        println!("You are logged in as a User!")
                    }
                };
                break;
            }
            Some(LoginAction::Denied) => {
                println!("Username or Password incorrect! 🤷‍♂️");
                tries += 1;
                if tries > 3 {
                    println!("Your account is locked, please contact the admin!");
                    break;
                }
            }
            None => {
                println!("You need to create an account!");
                break;
            }
        };

        // if login(&username, &password) {
        //     println!("{}", greet(&username));
        //     break;
        // } else {
        //     println!("Username or Password incorrect! 🤷‍♂️");
        //     tries += 1;
        //     if tries > 3 {
        //         println!("Your account is locked, please contact the admin!");
        //         break;
        //     }
        // }
    }
}

fn read_input() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error reading stdin!");
    buf.trim().to_string()
}
