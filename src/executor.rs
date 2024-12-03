use anyhow::{anyhow, Result};
use serde_json::Value;
use wasmtime::*;
use wasmtime_wasi::{add_to_linker, WasiCtx, WasiCtxBuilder};

// Import the host module
use crate::host;

pub struct Executor {
    engine: Engine,
    linker: Linker<WasiCtx>,
}

impl Executor {
    /// Create a new Executor with reusable environment
    pub fn new() -> Result<Self> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        // Add WASI functions to the linker
        add_to_linker(&mut linker, |ctx| ctx)?; // Modified closure

        // Add the custom `log` function to the linker
        linker.func_wrap("env", "log", host::Host::log)?;

        Ok(Self { engine, linker })
    }

    /// Execute a WebAssembly module with the specified function and arguments
    pub fn execute(
        &self,
        wasm_binary: &[u8],
        args: &[Val],
    ) -> Result<Value> {
        // Create a new WASI context
        let wasi_ctx = WasiCtxBuilder::new().build();

        // Create a new Store for this execution
        let mut store = Store::new(&self.engine, wasi_ctx);

        // Compile the module
        let module = Module::new(&self.engine, wasm_binary)?;

        // Instantiate the module
        let instance = self.linker.instantiate(&mut store, &module)?;

        // Get the function from the module
        let func = instance.get_func(&mut store, "run")
            .ok_or_else(|| anyhow!("Function `{}` not found", "run"))?;

        // Prepare the arguments
        let mut results = vec![Val::null(); func.ty(&store).results().len()];

        // Call the function with the provided arguments
        func.call(&mut store, args, &mut results)?;

        // Assuming the function returns a single i32 result
        if let Some(Val::I32(result)) = results.get(0) {
            Ok(serde_json::json!({ "result": result }))
        } else {
            Ok(serde_json::json!({ "result": null }))
        }
    }
}
