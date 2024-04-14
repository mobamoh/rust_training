use std::time::Instant;

use rayon::prelude::*;
fn main() {
    println!("Making thing easy with Rayon! 🦀🤗");

    println!("{:+<50}", "+ Sum: ");
    let numbers = (1..1_000_000).collect::<Vec<u64>>();
    let sum = numbers.par_iter().sum::<u64>();
    println!("The sum = {sum}");

    println!("{:+<50}", "+ is prime? : ");
    let now = Instant::now();
    let nums = (0..1000).collect::<Vec<u32>>();
    let mut primes = nums
        .par_iter()
        .filter(|n| is_prime(**n))
        .collect::<Vec<&u32>>();
    primes.par_sort_unstable();
    let elapsed = now.elapsed().as_secs_f32();
    println!("Found {} primes in {} seconds", primes.len(), elapsed);
}

fn is_prime(num: u32) -> bool {
    (2..=num / 2).into_par_iter().all(|i| num % i != 0)
}
