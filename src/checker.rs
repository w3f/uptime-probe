extern crate hyper;
extern crate hyper_tls;

use hyper::{Client, Response, Body};
use hyper_tls::HttpsConnector;

use crate::config;
use prometheus::{IntGaugeVec};

use std::error::Error;
use std::collections::HashMap;

lazy_static! {
    static ref INT_GAUGE_VECT: IntGaugeVec = register_int_gauge_vec!(
        "uptime_probe_errors",
        "uptime probe requests errors",
        &["result", "url", "scope"]).unwrap();
}

pub struct Checker {
    sites: Vec<config::Site>,
    allow_redirections: bool,
    prometheus_rule_scope: String,
    errors: HashMap<(String, String), bool>
}

impl Checker {
    pub fn new(sites: Vec<config::Site>, allow_redirections: bool, prometheus_rule_scope: String) -> Checker {
        let errors: HashMap<(String, String), bool> = HashMap::new();

        Checker { sites, allow_redirections, prometheus_rule_scope, errors }
    }
    fn handle_success(&mut self, resp: &Response<Body>, url: &str) {
        println!("SUCCESS: {} for {}", resp.status(), url);
        let prom_scope = self.prometheus_rule_scope.as_str();

        for ((key, code), _) in &self.errors {
            if key == url {
                println!("unsetting error {}, {}", code, url);
                INT_GAUGE_VECT.with_label_values(&[code.as_str(), url, prom_scope]).set(0);
            }
        }

        self.errors.retain(|(key, _), _| {
            key != url
        });

        INT_GAUGE_VECT.with_label_values(&["connection error", url, prom_scope]).set(0);
    }
    fn handle_failure(&mut self, code: &str, url: &str) {
        self.errors.insert((url.to_string(), code.to_string()), true);
        INT_GAUGE_VECT.with_label_values(&[code, url, self.prometheus_rule_scope.as_str()]).set(1);
    }
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);   

        // Get arounds Rust's borrowing rules. MAKE SURE to add `sites` back to
        // `self.sites` before the method returns.
        let sites = std::mem::take(&mut self.sites);

        for site in &sites {
            let url = site.url.as_str();
            println!("Processing {}", url);

            let uri = url.parse().unwrap();
            let res = client.get(uri).await;

            match res {
                Ok(resp) => {
                    if self.allow_redirections && resp.status().is_redirection() {
                            self.handle_success(&resp, url);
                    }
                    else{
                        match resp.status().as_str() {
                            "200" | "301" | "308" => {
                                self.handle_success(&resp, url);
                            },
                            code => {
                                println!("FAILURE: {} for {}", code, url);
                                self.handle_failure(code, url);
                            }
                        }

                    }

                },
                Err(e) => {
                    println!("FAILURE: connection error for {}: {}", url, e);
                    self.handle_failure("connection error", url);
                }
            }
        }

        // Add sites back to inner field.
        self.sites = sites;

        Ok(())
    }
}
