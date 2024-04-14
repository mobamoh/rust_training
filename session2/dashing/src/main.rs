use std::{thread, time::Duration};

use dashmap::DashMap;
use once_cell::sync::Lazy;

static SHARED: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);
fn main() {
    println!("🦀 DashMap 📍");

    for i in 0..100 {
        thread::spawn(move || loop {
            if let Some(mut v) = SHARED.get_mut(&i) {
                *v += 1;
            } else {
                SHARED.insert(i, i);
            }
        });
    }

    thread::sleep(Duration::from_secs(2));
    println!("{SHARED:#?}");
}
