use std::collections::HashSet;
use std::sync::OnceLock;
use crate::cli::CliArgs; // Import the CliArgs structure

#[derive(Debug)] // Automatically implements Debug for Config
pub struct Config {
    pub host: String,         // Hostname or IP
    pub port: u16,            // Port number
    pub ttl: u64,             // Time-to-live in milliseconds
    pub log_topics: HashSet<String>, // Logging topics
}

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Initialize the global configuration
pub fn init_config(args: CliArgs) {
    CONFIG
        .set(Config {
            host: args.host,
            port: args.port,
            ttl: args.ttl,
            log_topics: args.log.into_iter().collect(), // Convert Vec<String> to HashSet<String>
        })
        .expect("Config has already been initialized!");
}

/// Access the global configuration
pub fn global_config() -> &'static Config {
    CONFIG.get().expect("Config has not been initialized!")
}
