extern crate iron;

use std::error::Error;
use iron::prelude::*;
use iron::status;

extern crate router;
use router::Router;

use crate::metrics;

pub struct Server<'a>  {
    metrics: &'a metrics::Metrics,
}

impl<'a> Server<'a> {
    pub fn new(port: u32, metrics: &'a metrics::Metrics) -> Result<Server<'a>, Box<Error>> {
        let mut router = Router::new();
        router.get("/healthcheck", Self::get_healthcheck, "healthcheck");
        router.get("/metrics", Self::get_metrics, "metrics");

        println!("Serving on http://localhost:{}...", port);

        Iron::new(router).http(format!("0.0.0.0:{}", port))?;

        Ok(Server{ metrics })
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
        response.set_mut("Metrics!");
        Ok(response)
    }
}
