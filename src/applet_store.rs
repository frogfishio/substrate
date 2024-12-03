use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Metadata associated with each applet
#[derive(Clone, Debug)]
pub struct AppletMetadata {
    pub name: String,    // Name of the applet
    pub size: usize,     // Size of the wasm file in bytes
    pub created_at: u64, // Timestamp when the applet was stored
}

/// AppletStore to manage storage and retrieval of wasm files
#[derive(Clone)]
pub struct AppletStore {
    store: Arc<Mutex<HashMap<Uuid, (Vec<u8>, AppletMetadata)>>>, // Map UUID to (wasm binary, metadata)
}

impl AppletStore {
    /// Create a new AppletStore
    pub fn new() -> Self {
        AppletStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Store a new applet, returning its UUID
    pub fn create(&self, wasm_binary: Vec<u8>, name: String) -> Uuid {
        let uuid = Uuid::new_v4();
        let metadata = AppletMetadata {
            name,
            size: wasm_binary.len(),
            created_at: Self::current_timestamp(),
        };

        let mut store = self.store.lock().unwrap();
        store.insert(uuid, (wasm_binary, metadata));
        uuid
    }

    /// Retrieve a wasm binary and metadata by UUID
    pub fn get(&self, uuid: &Uuid) -> Option<(Vec<u8>, AppletMetadata)> {
        let store = self.store.lock().unwrap();
        store.get(uuid).cloned()
    }

    /// Helper function to get the current timestamp (UNIX epoch)
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        now.as_secs()
    }
}
