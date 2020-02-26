#![feature(async_closure)]
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

pub async fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let port = cfg.port;
    let handle = thread::spawn(async move || {
        let mut checker = checker::Checker::new(cfg.sites);

        let period_duration = time::Duration::from_secs(cfg.period as u64);
        loop {
            let now = time::Instant::now();
            println!("Running checker...");
            match checker.run().await {
                Ok(_) => {}
                Err(_) => {}
            }
            thread::sleep(period_duration - now.elapsed());
        }
    });

    let server_handle = thread::spawn(move || {
        server::Server::new(port).unwrap();
    });

    handle.join().unwrap().await;
    server_handle.join().unwrap();

    Ok(())
}
