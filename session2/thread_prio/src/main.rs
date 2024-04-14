use std::{sync::atomic::AtomicU32, time::Duration};
use thread_priority::*;

static LOW_COUNTER: AtomicU32 = AtomicU32::new(0);
static MAX_COUNTER: AtomicU32 = AtomicU32::new(0);

fn low_prio() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Release);
        std::thread::yield_now();
    }
}

fn max_prio() {
    set_current_thread_priority(ThreadPriority::Max).unwrap();
    loop {
        MAX_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Release);
        std::thread::yield_now();
    }
}

fn main() {
    println!("🦀 Thread priority 🏄‍♂️");

    std::thread::spawn(low_prio);
    std::thread::spawn(max_prio);

    std::thread::sleep(Duration::from_secs(5));

    println!("LOW: {}",LOW_COUNTER.load(std::sync::atomic::Ordering::Relaxed));
    println!("MAX: {}",MAX_COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
