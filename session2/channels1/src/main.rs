use std::sync::mpsc;

fn main() {
    println!("🦀 Channels (Muti-Producers - Single Consumer) 📺");

    // transmitter - receiver
    let (tx, rx) = mpsc::channel::<Command>();

    let thread = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello from Channel!"),
                Command::Quit => {
                    println!("Quiting the channel");
                    break;
                }
            }
        }
    });

    // thread.join().unwrap(); // dead lock
    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap();
    }
    tx.send(Command::Quit).unwrap();
    thread.join().unwrap();
}

enum Command {
    SayHello,
    Quit,
}
