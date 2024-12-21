mod cli; // CLI module
mod config; // Config module
mod applet_store; // Applet store module
mod net; // Networking module
mod types;
mod log;
mod executor;
mod host;
mod runner;

use cli::parse_args;
use config::init_config;
use applet_store::AppletStore;
use std::sync::Arc;
use std::process;

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args = parse_args();

    // Initialize global configuration with CLI arguments
    init_config(args.clone());

    // Access the global configuration (optional logging/debugging)
    let config = config::global_config();
    log::log("substrate", "Substrate starting up");

    // Set up applet store
    let store = Arc::new(AppletStore::new());

    // Check if a WASM file is provided
    if let Some(filename) = args.load.as_deref() {
        log::log("substrate", &format!("Loading WASM file: {}", filename));
        let wasm_binary = match std::fs::read(filename) {
            Ok(data) => data,
            Err(e) => {
                log::log(
                    "substrate",
                    &format!("Failed to read the WASM file '{}': {}", filename, e),
                );
                shutdown(1, "Failed to read the WASM file");
                return; // Ensure the program doesn't continue
            }
        };
        let uuid = store.create(wasm_binary, "Loaded Applet".to_string());
        log::log("substrate", &format!("Applet stored with UUID: {}", uuid));
    } else {
        log::log("substrate", "No WASM file specified. Shutting down.");
        shutdown(1, "No WASM file specified");
    }

    // Start the server using net.rs
    net::start_server(store).await;
}

/// Helper function to handle shutdown with logging and exit code
fn shutdown(exit_code: i32, reason: &str) {
    log::log("substrate", &format!("Shutting down: {}", reason));
    process::exit(exit_code);
}
