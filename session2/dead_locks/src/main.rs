use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn main() {
    // let my_shared = Mutex::new(0);

    // dead lock
    // let lock = my_shared.lock().unwrap();
    // let lock = my_shared.lock().unwrap();

    // fix with scope
    // {
    //     let lock = my_shared.lock().unwrap();
    // }
    // let lock = my_shared.lock().unwrap();

    // fix with try lock
    // let lock = MY_SHARED.lock().unwrap();
    // // std::mem::drop(lock); // drops the lock
    // if let Ok(_lock) = MY_SHARED.try_lock() {
    //     println!("Got the lock!")
    // } else {
    //     println!("No lock!")
    // }

    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");


    let recovered_data = lock.unwrap_or_else(|poisoned| {
        println!("Mutex was poisone, recovering data...");
        poisoned.into_inner()
    });
    println!("{:?}",recovered_data);
}

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("poinsoning..")
}
