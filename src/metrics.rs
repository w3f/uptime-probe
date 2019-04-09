use prometheus::{IntCounterVec};

lazy_static! {
    static ref INT_COUNTER_VECT: IntCounterVec = register_int_counter_vec!("uptime_probe_results", "uptime probe requests results", &["result", "url", "missed_needle"]).unwrap();
}
