extern crate hyper;
extern crate hyper_tls;

use hyper::Client;
use hyper::rt::{self, Future};
use hyper_tls::HttpsConnector;

use crate::config;
use prometheus::{IntGaugeVec};

use std::error::Error;

lazy_static! {
    static ref INT_GAUGE_VECT: IntGaugeVec = register_int_gauge_vec!(
        "uptime_probe_errors",
        "uptime probe requests errors",
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
            let url_success = site.url.clone();
            let url_err = site.url.clone();
            let uri = site.url.parse().unwrap();
            // let needles = site.needles.clone();
            rt::run(rt::lazy(|| {
                let https = HttpsConnector::new(4).unwrap();
                let client = Client::builder()
                    .build::<_, hyper::Body>(https);

                client
                    .get(uri)
                    .map(move |res| {
                        match res.status().as_str() {
                            "200" => INT_GAUGE_VECT.with_label_values(&["200", &url_success[..], ""]).set(0),
                            s => INT_GAUGE_VECT.with_label_values(&[s, &url_success[..], ""]).set(1),
                        }
                    })
                    .map_err(move |_| {
                        INT_GAUGE_VECT.with_label_values(&["connection error", &url_err[..], ""]).set(1);
                    })
            }));
        }
        Ok(())
    }
}
