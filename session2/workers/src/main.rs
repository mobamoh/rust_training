use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn main() {
    println!("Worker Threads 👷");
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                Command::Run(job) => job(),
                Command::Quit => {
                    println!("Quitting...");
                    break;
                }
            }
        }
    });

    let greet_closure = || println!("Hello from closure");
    let count_closure = || {
        for i in 1..5 {
            println!("Hello from counter closure {i}")
        }
    };

    tx.send(Command::Run(Box::new(greet_closure))).unwrap();
    tx.send(Command::Run(Box::new(count_closure))).unwrap();
    tx.send(Command::Run(Box::new(|| println!("inline Closure"))))
        .unwrap();
    tx.send(Command::Run(Box::new(greet))).unwrap(); // calling static function

    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}

fn greet() {
    println!("greetings!!!")
}
