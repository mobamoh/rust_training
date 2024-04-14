fn main() {
    println!("🦀 Rayon Scopes 🔎");

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello from pool thread!"));

    pool.scope(|scope| {
        for i in 0..10 {
            scope.spawn(move |_| {
                println!("Hello from scoped thread {i}");
            });
        }
    });

    println!("Hello from main thread!");

    println!("{:+<50}", "");

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, broadcast_context| {
            println!("Hello from broadcast thread {}", broadcast_context.index());
        });
    });
}
