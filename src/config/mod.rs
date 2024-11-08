use std::env;

pub struct GlobalConfig {
    pub prometheus: PrometheusConfig,
    pub port: i32,
}

impl GlobalConfig {
    pub fn new() -> Self {
        // Setting default listening device.
        let listen_to = env::var("PORT")
            .unwrap_or("8080".to_string())
            .trim()
            .parse::<i32>()
            .unwrap();

        GlobalConfig {
            prometheus: PrometheusConfig::new(),
            port: listen_to,
        }
    }
}

pub struct PrometheusConfig {
    pub port: i32,
}

impl PrometheusConfig {
    pub fn new() -> Self {
        let prom_port = match env::var("PROM_PORT") {
            Ok(val) => val.trim().parse().unwrap(),
            //If nothing is found or error occured, set default port.
            Err(_) => {
                println!("Unable to parse prometheus port from environment variable. Setting default port to 3030");
                3030
            }
        };
        PrometheusConfig { port: prom_port }
    }
}
