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
        for (k, _) in &self.errors {
            if k.0 == url {
                println!("unsetting error {}, {}", k.1, url);
                INT_GAUGE_VECT.with_label_values(&[k.1.as_str(), url, self.prometheus_rule_scope.as_str()]).set(0);
            }
        }
        self.errors.retain(|key, _| {
            key.0 != url
        });
        INT_GAUGE_VECT.with_label_values(&["connection error", url, self.prometheus_rule_scope.as_str()]).set(0);
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

                    if self.allow_redirections && r.status().is_redirection() {
                            println!("SUCCESS: {} for {}", r.status(), u);
                            for (k, _) in &self.errors {
                                if k.0 == &site.url[..] {
                                    println!("unsetting error {}, {}", k.1, u);
                                    INT_GAUGE_VECT.with_label_values(&[k.1.as_str(), u, self.prometheus_rule_scope.as_str()]).set(0);
                                }
                            }
                            self.errors.retain(|key, _| {
                                !(key.0 == &site.url[..])
                            });
                            INT_GAUGE_VECT.with_label_values(&["connection error", &site.url[..], self.prometheus_rule_scope.as_str()]).set(0);
                    }
                    else{

                        match r.status().as_str() {
                            "200" | "301" | "308" => {
                                println!("SUCCESS: {} for {}", r.status(), u);
                                for (k, _) in &self.errors {
                                    if k.0 == &site.url[..] {
                                        println!("unsetting error {}, {}", k.1, u);
                                        INT_GAUGE_VECT.with_label_values(&[k.1.as_str(), u, self.prometheus_rule_scope.as_str()]).set(0);
                                    }
                                }
                                self.errors.retain(|key, _| {
                                    !(key.0 == &site.url[..])
                                });
                                INT_GAUGE_VECT.with_label_values(&["connection error", &site.url[..], self.prometheus_rule_scope.as_str()]).set(0);
                            },
                            s => {
                                println!("FAILURE: {} for {}", s, u);
                                self.errors.insert((u.to_string(), s.to_string()), true);
                                INT_GAUGE_VECT.with_label_values(&[s, u, self.prometheus_rule_scope.as_str()]).set(1);
                            }
                        }

                    }

                },
                Err(e) => {
                    let u = &site.url[..];
                    println!("FAILURE: connection error for {}: {}", u, e);
                    self.errors.insert((u.to_string(), "connection error".to_string()), true);
                    INT_GAUGE_VECT.with_label_values(&["connection error", &site.url[..], self.prometheus_rule_scope.as_str()]).set(1);
                }
            }
        }
        Ok(())
    }
}
