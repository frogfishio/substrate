use clap::Parser;

/// Command-line arguments for the application
#[derive(Parser, Debug, Clone)] // Added `Clone` here
#[command(name = "WASM Server")]
#[command(about = "A server for hosting and managing WASM applets", long_about = None)]
pub struct CliArgs {
    /// Hostname or IP address to bind the server
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Port number for the server
    #[arg(long, default_value = "3030")]
    pub port: u16,

    /// Time-to-live for applets in milliseconds
    #[arg(long, default_value = "60000")]
    pub ttl: u64,

    /// WASM file to load and execute
    #[arg(long)]
    pub load: Option<String>,

    /// Logging topics (comma-separated list)
    #[arg(long, value_delimiter = ',', use_value_delimiter = true)]
    pub log: Vec<String>,
}

/// Parse and return the command-line arguments
pub fn parse_args() -> CliArgs {
    let mut args = CliArgs::parse();

    // Ensure "substrate" is always included in the logging topics
    let mut topics: std::collections::HashSet<String> = args.log.into_iter().collect();
    topics.insert("substrate".to_string());

    args.log = topics.into_iter().collect(); // Deduplicated and ensured "substrate"
    args
}
