fn main() {
    const N_THREADS: usize = 8;
    let numbers_to_add = (1..6000).collect::<Vec<u32>>();
    println!(
        "We have {} Workers assigned to {} numbers",
        N_THREADS,
        numbers_to_add.len()
    );

    let mut thread_handles = Vec::new();

    let chunks = numbers_to_add.chunks(N_THREADS);
    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        thread_handles.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()));
    }

    println!(
        "The sum of the numbers is: {}",
        thread_handles
            .into_iter()
            .map(|th| th.join().unwrap())
            .sum::<u32>()
    );
}
