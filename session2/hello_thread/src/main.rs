use std::ops::Add;

use chrono::Datelike;

fn hello_thread() {
    println!("Hello from the thread!")
}

fn guten_tag_thread(i: u32) {
    println!("Guten Tag from thread {i}!")
}

fn add_6_years(age: u32) -> (u32, u32) {
    (age, age + 6)
}

fn main() {
    println!("Hello from the main thread!");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap();
    println!("{:+<50}", "");

    let mut thread_handles = Vec::new();
    for i in 0..5 {
        // i is  closure
        // move is used to move ownership to the thread
        let handle = std::thread::spawn(move || guten_tag_thread(i));
        thread_handles.push(handle);
    }
    thread_handles.into_iter().for_each(|t| t.join().unwrap());
    println!("{:+<50}", "");

    println!(
        "{:<20}{:<20}",
        format!("{} in {}", "Age", chrono::Utc::now().year()),
        format!(
            "{} in 6 years in {}",
            "Age",
            chrono::Utc::now().year().add(6)
        )
    );
    println!("{:-<40}", "");
    let mut thread_handles = Vec::new();
    for i in 33..=39 {
        let thread_handle = std::thread::spawn(move || add_6_years(i));
        thread_handles.push(thread_handle);
    }
    thread_handles.into_iter().for_each(|th| {
        let age = th.join().unwrap();
        println!("{:<20}{:<20}", age.0, age.1);
    })
}
