use std::env;
use std::process;
use uptime_probe::{Config, run};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    match run(config).await {
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
        Ok(_) => {}
    }
}
