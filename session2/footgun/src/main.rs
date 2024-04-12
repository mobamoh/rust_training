use std::{sync::atomic::AtomicU32, thread};
static mut COUNTER: u32 = 0;
static COUNTER_ATOMIC: AtomicU32 = AtomicU32::new(0);

fn main() {
    println!("Footgun 🦶🔫");

    println!("{:+<50}", "");
    let mut counter = 0;
    for _ in 0..1000 {
        for _ in 1..11000 {
            counter += 1;
        }
    }
    println!("The single threaded sum: {}", counter);
    // println!(
    //     "The single threaded sum of 1..9 is: {}",
    //     (1..10).collect::<Vec<u32>>().into_iter().sum::<u32>()
    // );

    println!("{:+<50}", "");
    let mut thread_handles = Vec::new();
    for _ in 0..1000 {
        let thread_handle = thread::spawn(move || {
            for _ in 1..11000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|th| th.join().unwrap());
    unsafe {
        println!("The multi-threaded sum: {}", COUNTER);
    }

    println!("{:+<50}", "");

    let mut thread_handles = Vec::new();
    for _ in 0..1000 {
        let thread_handle = thread::spawn(move || {
            for _ in 1..11000 {
                COUNTER_ATOMIC.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|th| th.join().unwrap());
    println!(
        "The atomic sum: {}",
        COUNTER_ATOMIC.load(std::sync::atomic::Ordering::Relaxed)
    );
}
