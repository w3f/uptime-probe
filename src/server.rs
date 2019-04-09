extern crate iron;

use prometheus::{TextEncoder, Encoder};

use std::error::Error;
use iron::prelude::*;
use iron::status;

extern crate router;
use router::Router;

pub struct Server  {
}

impl Server {
    pub fn new(port: u32) -> Result<Server, Box<Error>> {
        let mut router = Router::new();
        router.get("/healthcheck", Self::get_healthcheck, "healthcheck");
        router.get("/metrics", Self::get_metrics, "metrics");

        println!("Serving on http://localhost:{}...", port);

        Iron::new(router).http(format!("0.0.0.0:{}", port))?;

        Ok(Server{})
    }

    fn get_healthcheck(_request: &mut Request) -> IronResult<Response> {
        let mut response = Response::new();
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset=Utf8));
        response.set_mut("Ok!");
        Ok(response)
    }

    fn get_metrics(_request: &mut Request) -> IronResult<Response> {
        let mut response = Response::new();
        response.set_mut(status::Ok);
        response.set_mut(mime!(Text/Html; Charset=Utf8));

        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        response.set_mut(String::from_utf8(buffer).unwrap());
        Ok(response)
    }
}
