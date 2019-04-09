use prometheus::{Opts, Registry, CounterVec, TextEncoder, Encoder};
use std::error::Error;

pub struct Metrics {
    registry: Registry,
    counter: CounterVec,
}

impl Metrics {
    pub fn new() -> Metrics {
        let counter_opts = Opts::new("uptime_probe_results", "uptime probe requests results")
            .const_label("result", "")
            .const_label("url", "")
            .const_label("missed_needle", "");
        let counter = CounterVec::new(counter_opts, &["", "", ""]).unwrap();

        let registry = Registry::new();
        registry.register(Box::new(counter.clone())).unwrap();

        Metrics { registry, counter }
    }

    pub fn inc_counter(&self, result: &str, url: &str, missed_needle: &str) {
        self.counter.with_label_values(&[result, url, missed_needle]).inc();
    }

    pub fn render(&self) -> Result<Vec<u8>, Box<dyn Error>>{
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        Ok(buffer)
    }
}
