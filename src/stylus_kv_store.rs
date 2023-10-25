use std::collections::HashMap;
use log::info;

pub struct StylusKVStore {
    store: HashMap<String, String>,
}

impl StylusKVStore {
    pub fn new() -> Self {
        StylusKVStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        info!("Set {} = {}", &key, &value);
        self.store.insert(key, value)
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &String) -> Option<String> {
        info!("Deleted key: {}", &key);
        self.store.remove(key)
    }

    pub fn contains(&self, key: &String) -> bool {
        self.store.contains_key(key)
    }
}
