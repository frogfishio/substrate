use warp::Filter;
use warp::http::{Method, HeaderMap};
use std::sync::Arc;
use uuid::Uuid;
use crate::{applet_store::AppletStore, runner::Runner, log, config};
use crate::types::HttpRequest; // Import the custom request struct
use bytes::Bytes;
use std::net::IpAddr; // Import IpAddr

pub async fn start_server(store: Arc<AppletStore>) {
    // Access the global configuration
    let config = config::global_config();

    let wasm_runner = Arc::new(Runner::new(store.clone()));

    // Define a route for handling all requests
    let handle_request = {
        let wasm_runner = wasm_runner.clone();
    
        warp::path::param::<Uuid>() // Match a UUID in the path
            .and(warp::method()) // Capture the HTTP method
            .and(warp::header::headers_cloned()) // Clone all request headers
            .and(warp::header::optional("cookie")) // Capture the Cookie header if present
            .and(warp::path::full()) // Capture the full path
            .and(warp::query::raw().or(warp::any().map(|| "".to_string())).unify()) // Handle missing query strings
            .and(warp::body::bytes()) // Capture the entire request body as raw bytes
            .and(warp::filters::addr::remote()) // Capture the remote client's IP address
            .map(
                move |uuid: Uuid,
                      method: Method,
                      headers: HeaderMap,
                      cookies: Option<String>,
                      full_path: warp::filters::path::FullPath,
                      query_string: String, // Query string is now guaranteed
                      body: Bytes,
                      remote_addr: Option<std::net::SocketAddr>| {
                    // Populate the custom HttpRequest struct
                    let request = HttpRequest {
                        method,
                        headers,
                        cookies,
                        path: full_path.as_str().to_string(),
                        query: query_string, // Query string is now safe
                        body,
                        remote_addr,
                    };
    
                    // Delegate to the WASM runner
                    match wasm_runner.run(uuid, request) {
                        Ok(response) => warp::reply::with_status(
                            warp::reply::json(&response),
                            warp::http::StatusCode::OK,
                        ),
                        Err(err) => warp::reply::with_status(
                            warp::reply::json(&err),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ),
                    }
                },
            )
    };    

    // Parse the host string into an IpAddr
    let host: IpAddr = config.host.parse().expect("Invalid host");

    // Start the server with the parsed host and port from the configuration
    log::log(
        "substrate",
        &format!(
            "Substrate server running at http://{}:{}",
            host, config.port
        ),
    );
    warp::serve(handle_request)
        .run((host, config.port))
        .await;
}
