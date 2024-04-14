use std::sync::Mutex;
static SHARED_VEC: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    println!("🦀 Mutexes and Locks 🔐");

    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            let mut lock = SHARED_VEC.lock().unwrap();
            lock.push(1);
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());

    let lock = SHARED_VEC.lock().unwrap();
    println!("{:#?}", lock);
}
