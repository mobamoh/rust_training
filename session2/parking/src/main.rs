fn main() {
    println!("Parking 🎢");

    let mut threads = Vec::new();
    for i in 0..10 {
        let thread = std::thread::spawn(move || {
            parkable_thread(i);
        });
        threads.push(thread);
    }

    loop {
        println!("Thread to unpark (q/Q to quite):");
        let input = read_line();
        if input.to_lowercase() == "q" {
            break;
        }
        if let Ok(n) = input.parse::<usize>() {
            if n < 10 {
                threads.get(n).unwrap().thread().unpark();
            }
        }
    }
}

fn parkable_thread(i: u32) {
    loop {
        std::thread::park();
        println!("Thread {i} is unparked, briefly");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error reading stdin");
    input.trim().to_string()
}
