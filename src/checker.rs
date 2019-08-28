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
        &["result", "url"]).unwrap();
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
            let mut errors: Vec<String> = vec![];
            rt::run(rt::lazy(|| {
                let https = HttpsConnector::new(4).unwrap();
                let client = Client::builder()
                    .build::<_, hyper::Body>(https);
                client
                    .get(uri)
                    .map(move |res| {
                        let u = &url_success[..];
                        match res.status().as_str() {
                            "200" | "301" => {
                                println!("SUCCESS: {} for {}", res.status(), u);
                                for error in errors.iter() {
                                    INT_GAUGE_VECT.with_label_values(&[&error, u]).set(0);
                                }
                                INT_GAUGE_VECT.with_label_values(&["connection error", &url_success[..]]).set(0);
                            },
                            s => {
                                println!("FAILURE: {} for {}", s, u);
                                errors.push(s.to_string().clone());
                                INT_GAUGE_VECT.with_label_values(&[s, u]).set(1);
                            }
                        }
                    })
                    .map_err(move |_| {
                        let u = &url_err[..];
                        println!("FAILURE: connection error for {}", u);
                        INT_GAUGE_VECT.with_label_values(&["connection error", &url_err[..]]).set(1);
                    })
            }));
        }
        Ok(())
    }
}
