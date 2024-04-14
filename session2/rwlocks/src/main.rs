use std::{sync::RwLock, thread::spawn};

use once_cell::sync::Lazy;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(init_users()));

fn main() {
    println!("🔖 ✍️ 🔐 Read/Write Locks with Lazy Init 🦦");

    std::thread::spawn(|| loop {
        println!("Current users (in the thread)");
        let users = USERS.read().unwrap();
        println!("{users:?}");
        std::thread::sleep(std::time::Duration::from_secs(3));
    });

    loop {
        println!("Enter a name to add to the user's list (q/Q to quit the program)");
        let input = read_line();
        if input.to_lowercase() == "q" {
            break;
        }
        USERS.write().unwrap().push(input);
    }
}

fn init_users() -> Vec<String> {
    vec!["Moha".to_string(), "Driss".to_string()]
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("error reading stdin.");
    buf.trim().to_string()
}
