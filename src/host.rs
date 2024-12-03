use anyhow::{anyhow, Result};
use wasmtime::{Caller, Memory, Extern};
use wasmtime_wasi::WasiCtx;

// Import your log module
use crate::log;

pub struct Host;

impl Host {
    /// Host function to log messages from WASM
    pub fn log(
        mut caller: Caller<'_, WasiCtx>,
        topic_ptr: i32,
        topic_len: i32,
        msg_ptr: i32,
        msg_len: i32,
    ) -> Result<()> {
        // Retrieve the memory export
        let memory = match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => mem,
            _ => return Err(anyhow!("Failed to find memory")),
        };

        // Read the topic and message strings from the memory
        let topic = Self::read_string_from_memory(&memory, &mut caller, topic_ptr, topic_len)?;
        let message = Self::read_string_from_memory(&memory, &mut caller, msg_ptr, msg_len)?;

        // Call the log function from log.rs
        log::log(&topic, &message);

        Ok(())
    }

    /// Helper to read a string from WASM memory
    fn read_string_from_memory(
        memory: &Memory,
        caller: &mut Caller<'_, WasiCtx>,
        ptr: i32,
        len: i32,
    ) -> Result<String> {
        let data = memory.data(&caller);
        let start = ptr as usize;
        let end = start.checked_add(len as usize)
            .ok_or_else(|| anyhow!("Integer overflow when calculating string bounds"))?;

        if end > data.len() {
            return Err(anyhow!("Pointer and length out of bounds"));
        }

        let bytes = &data[start..end];
        let s = std::str::from_utf8(bytes)?.to_string();
        Ok(s)
    }
}
