#[macro_use] extern crate prometheus;

#[macro_use] extern crate serde_derive;

#[macro_use] extern crate mime;

#[macro_use] extern crate lazy_static;

use std::error::Error;
use std::thread;
use std::time;

pub use config::Config;

mod config;
mod checker;
mod server;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let port = cfg.port;
    let handle = thread::spawn(move || {
        let checker = checker::Checker::new(cfg.sites);

        let period_duration = time::Duration::from_secs(cfg.period as u64);
        loop {
            let now = time::Instant::now();
            println!("Running checker...");
            checker.run().unwrap();
            thread::sleep(period_duration - now.elapsed());
        }
    });

    let server_handle = thread::spawn(move || {
        server::Server::new(port).unwrap();
    });

    handle.join().unwrap();
    server_handle.join().unwrap();

    Ok(())
}
