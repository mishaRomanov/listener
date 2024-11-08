use log;
use std::env;

#[derive(Debug)]
pub struct PrometheusConfig {
    pub port: i32,
}

impl PrometheusConfig {
    pub fn new() -> Self {
        let prom_port = match env::var("PROM_PORT") {
            Ok(val) => val.trim().parse().unwrap(),
            //If nothing is found or error occured, set default port.
            Err(_) => {
                log::warn!("Unable to parse prometheus port from environment variable. Setting default port to 3030");
                3030
            }
        };

        PrometheusConfig { port: prom_port }
    }
}
