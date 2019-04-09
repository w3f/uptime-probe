use crate::config;
use crate::metrics;

use std::error::Error;

pub struct Checker {
    sites: Vec<config::Site>,
}

impl Checker {
    pub fn new(sites: Vec<config::Site>) -> Checker {
        Checker { sites }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        for site in &self.sites {
            println!("Processing {}", site.url)
        }

        Ok(())
    }
}
