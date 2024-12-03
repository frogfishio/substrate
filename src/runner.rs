use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use serde_json::Value;
use crate::applet_store::AppletStore;
use crate::types::{HttpRequest, HttpResponse};
use crate::executor::Executor;

/// Runner for executing WebAssembly applets with caching
pub struct Runner {
    store: Arc<AppletStore>,
    cache: Mutex<HashMap<Uuid, Arc<Executor>>>, // Cache for compiled executors
}

impl Runner {
    /// Creates a new Runner
    pub fn new(store: Arc<AppletStore>) -> Result<Self> {
        Ok(Self {
            store,
            cache: Mutex::new(HashMap::new()),
        })
    }

    /// Executes the applet identified by UUID with the given request
    pub fn run(&self, uuid: Uuid, request: HttpRequest) -> Result<HttpResponse> {
        // Get or cache the executor
        let executor = self.get_or_cache_executor(uuid)?;

        // Prepare arguments for execution
        let args = self.prepare_args(&request)?;

        // Execute the module and collect the result
        let result = executor.execute(request.body.as_ref(), &args)?;

        // Convert the result into an HttpResponse
        self.prepare_response(result)
    }

    /// Gets the cached executor or caches a new one if not already stored
    fn get_or_cache_executor(&self, uuid: Uuid) -> Result<Arc<Executor>> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(executor) = cache.get(&uuid) {
            return Ok(Arc::clone(executor)); // Return cached executor
        }

        // Fetch the Wasm binary from the applet store
        let wasm_binary = self
            .store
            .get(uuid)
            .ok_or_else(|| anyhow!("Applet not found for UUID: {}", uuid))?;

        // Create a new Executor instance
        let executor = Arc::new(Executor::new()?);

        // Cache the executor
        cache.insert(uuid, Arc::clone(&executor));
        Ok(executor)
    }

    /// Prepares arguments for the Wasm module execution
    fn prepare_args(&self, request: &HttpRequest) -> Result<Vec<Val>> {
        // Example: Convert the HTTP request body length to a single argument
        Ok(vec![Val::I32(request.body.len() as i32)])
    }

    /// Converts the Wasm module's result into an HttpResponse
    fn prepare_response(&self, result: Value) -> Result<HttpResponse> {
        Ok(HttpResponse {
            status_code: 200,
            body: serde_json::to_vec(&result)?,
            headers: Default::default(),
        })
    }
}
