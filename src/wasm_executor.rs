use crate::{
    applet_store::{AppletStore, AppletMetadata},
    types::HttpRequest,
};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use wasmtime::{Engine, Linker, Module, Store, Memory};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

pub struct Executor {
    applet_store: Arc<AppletStore>,
    engine: Engine,
}

impl Executor {
    /// Create a new WasmExecutor with a reference to the AppletStore
    pub fn new(applet_store: Arc<AppletStore>) -> Self {
        let engine = Engine::default();
        Self {
            applet_store,
            engine,
        }
    }

    /// Execute a request by retrieving the WASM binary and invoking its logic
    pub fn execute(&self, uuid: Uuid, request: HttpRequest) -> Result<Value, Value> {
        // Retrieve the WASM binary and metadata from the store
        let (wasm_binary, metadata) = self
            .applet_store
            .get(&uuid)
            .ok_or_else(|| serde_json::json!({ "error": "Applet not found" }))?;

        // Log metadata for debugging purposes
        println!("Executing applet: {:?}", metadata);

        // Delegate to the WASM runtime
        self.invoke_wasm(wasm_binary, request)
    }

    /// Invoke the WASM binary with the given request
    fn invoke_wasm(&self, wasm_binary: Vec<u8>, request: HttpRequest) -> Result<Value, Value> {
        // Compile the WASM module
        let module = Module::new(&self.engine, wasm_binary).map_err(|e| {
            serde_json::json!({ "error": format!("Failed to compile WASM module: {}", e) })
        })?;

        // Create a WASI context
        let wasi_ctx = WasiCtxBuilder::new().build();

        // Create a store with the WASI context
        let mut store = Store::new(&self.engine, wasi_ctx);

        // Create a linker and define the `log` function
        let mut linker = Linker::new(&self.engine);

        // Add `log` function to the linker
        linker
            .func_wrap(
                "env",
                "log",
                move |mut caller: wasmtime::Caller<'_, WasiCtx>, msg_ptr: i32, topic_ptr: i32| {
                    let memory = caller
                        .get_export("memory")
                        .and_then(|e| e.into_memory())
                        .ok_or_else(|| wasmtime::Trap::from("Memory not found"))?;

                    // Read message and topic strings
                    let message = Self::read_string_from_memory(&memory, &mut caller, msg_ptr)
                        .map_err(|e| wasmtime::Trap::from(e))?;
                    let topic = Self::read_string_from_memory(&memory, &mut caller, topic_ptr)
                        .map_err(|e| wasmtime::Trap::from(e))?;

                    // Log the message
                    println!("[{}] {}", topic, message);
                    Ok(())
                },
            )
            .map_err(|e| serde_json::json!({ "error": format!("Failed to define `log`: {}", e) }))?;

        // Instantiate the WASM module
        let instance = linker.instantiate(&mut store, &module).map_err(|e| {
            serde_json::json!({ "error": format!("Failed to instantiate WASM module: {}", e) })
        })?;

        // Get the `run` function from the WASM module
        let run_func = instance
            .get_typed_func::<i32, i32, _>(&mut store, "run")
            .map_err(|e| {
                serde_json::json!({ "error": format!("Failed to find `run` function: {}", e) })
            })?;

        // Serialize the `HttpRequest` into WASM memory
        let request_ptr = self.serialize_request(&mut store, &request)?;

        // Call the `run` function
        let result = run_func.call(&mut store, request_ptr).map_err(|e| {
            serde_json::json!({ "error": format!("Failed to execute `run` function: {}", e) })
        })?;

        // Decode the result from WASM memory
        self.decode_result(result)
    }

    /// Helper function to read a string from WASM memory
    fn read_string_from_memory(
        memory: &Memory,
        caller: &mut wasmtime::Caller<'_, WasiCtx>,
        ptr: i32,
    ) -> Result<String, String> {
        let data = memory.data_mut(caller);
        let len = data[ptr as usize] as usize;
        let bytes = &data[(ptr as usize + 1)..(ptr as usize + 1 + len)];
        String::from_utf8(bytes.to_vec()).map_err(|e| format!("Invalid UTF-8 string: {}", e))
    }

    /// Serialize the HttpRequest into WASM memory
    fn serialize_request(
        &self,
        store: &mut Store<WasiCtx>,
        request: &HttpRequest,
    ) -> Result<i32, Value> {
        // TODO: Serialize the HttpRequest into WASM memory and return a pointer
        Ok(0)
    }

    /// Decode the result of the WASM execution
    fn decode_result(&self, result: i32) -> Result<Value, Value> {
        // TODO: Decode the result from WASM memory
        Ok(serde_json::json!({ "status": "success", "result": result }))
    }
}
