use std::thread;
fn main() {
    println!("🦀 Scoped Threads 🚀");

    const N_THREADS: usize = 8;
    let numbers_to_add = (1..6000).collect::<Vec<u32>>();
    let chunks = numbers_to_add.chunks(N_THREADS);

    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();
        for chk in chunks {
            thread_handles.push(s.spawn(move || chk.iter().sum::<u32>()));
        }
        thread_handles
            .into_iter()
            .map(|th| th.join().unwrap())
            .sum::<u32>()
    });

    println!("The sum of the number is: {}", sum);
}
