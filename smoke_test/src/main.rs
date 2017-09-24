#[macro_use] extern crate log;
extern crate env_logger;

use std::thread;

fn main() {
    env_logger::init();

    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            info!("info from {}", i);
            error!("error from {}", i);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
