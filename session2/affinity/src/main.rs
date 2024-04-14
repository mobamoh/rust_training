fn main() {
    println!("Setting thread affinity!");

    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids
        .into_iter()
        .map(|id| {
            std::thread::spawn(move || {
                let success = core_affinity::set_for_current(id);
                if success {
                    println!("Hello from a thread on core {id:?}");
                } else {
                    println!("Unable to set affinity on core {id:?}");
                }
            })
        })
        .collect::<Vec<_>>();

    handles.into_iter().for_each(|h| h.join().unwrap());
}
