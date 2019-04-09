extern crate hyper;
extern crate hyper_tls;

use hyper::Client;
use hyper::rt::{self, Future};
use hyper_tls::HttpsConnector;

use crate::config;
use prometheus::{IntCounterVec};

use std::error::Error;

lazy_static! {
    static ref INT_COUNTER_VECT: IntCounterVec = register_int_counter_vec!(
        "uptime_probe_results",
        "uptime probe requests results",
        &["result", "url", "missed_needle"]).unwrap();
}

pub struct Checker {
    sites: Vec<config::Site>,
}

impl Checker {
    pub fn new(sites: Vec<config::Site>) -> Checker {
        Checker { sites }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {

        for site in &self.sites {
            println!("Processing {}", site.url);
            let uri = site.url.parse().unwrap();
            // let needles = site.needles.clone();
            rt::run(rt::lazy(|| {
                let https = HttpsConnector::new(4).unwrap();
                let client = Client::builder()
                    .build::<_, hyper::Body>(https);

                client
                    .get(uri)
                    .map(|res| {
                        println!("Response: {}", res.status());
                    })
                    .map_err(|err| {
                        println!("Error: {}", err);
                        // INT_COUNTER_VECT.with_label_values(&["error", uri, ""]).inc();
                    })
            }));
        }

        Ok(())
    }
}
