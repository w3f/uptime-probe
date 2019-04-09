#[macro_use] extern crate prometheus;

#[macro_use] extern crate serde_derive;

#[macro_use] extern crate mime;

use std::error::Error;
pub use config::Config;

//use checker;

mod config;
//mod checker;
mod server;
mod metrics;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let metrics = metrics::Metrics::new();
    //let checker = checker::Checker::new(cfg, metrics);
    let _server = server::Server::new(cfg.port, &metrics)?;

    Ok(())
}
