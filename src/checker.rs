extern crate hyper;
extern crate hyper_tls;

use hyper::Client;
use hyper_tls::HttpsConnector;

use crate::config;
use prometheus::{IntGaugeVec};

use std::error::Error;
use std::collections::HashMap;

lazy_static! {
    static ref INT_GAUGE_VECT: IntGaugeVec = register_int_gauge_vec!(
        "uptime_probe_errors",
        "uptime probe requests errors",
        &["result", "url"]).unwrap();
}

pub struct Checker {
    sites: Vec<config::Site>,
    errors: HashMap<(String, String), bool>
}

impl Checker {
    pub fn new(sites: Vec<config::Site>) -> Checker {
        let errors: HashMap<(String, String), bool> = HashMap::new();

        Checker { sites, errors }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        for site in &self.sites {
            println!("Processing {}", site.url);
            let uri = site.url.parse().unwrap();
            let res = client.get(uri).await;
            match res {
                Ok(r) => {
                    let u = &site.url[..];
                    match r.status().as_str() {
                        "200" | "301" => {
                            println!("SUCCESS: {} for {}", r.status(), u);
                            let key = &(u.to_string(), r.status().as_str().to_string());
                            if self.errors.contains_key(key) && *self.errors.get(key).unwrap() {
                                println!("unsetting error {}, {}", r.status(), u);
                                self.errors.remove(key);
                                INT_GAUGE_VECT.with_label_values(&[r.status().as_str(), u]).set(0);
                            }
                            INT_GAUGE_VECT.with_label_values(&["connection error", &site.url[..]]).set(0);
                        },
                        s => {
                            println!("FAILURE: {} for {}", s, u);
                            self.errors.insert((u.to_string(), s.to_string()), true);
                            INT_GAUGE_VECT.with_label_values(&[s, u]).set(1);
                        }
                    }
                },
                Err(e) => {
                    let u = &site.url[..];
                    println!("FAILURE: connection error for {}: {}", u, e);
                    self.errors.insert((u.to_string(), "connection error".to_string()), true);
                    INT_GAUGE_VECT.with_label_values(&["connection error", &site.url[..]]).set(1);
                }
            }
        }
        Ok(())
    }
}
