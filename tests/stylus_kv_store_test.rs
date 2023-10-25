// tests/kv_store_tests.rs

#[cfg(test)]
mod tests {
    use stylusdb::stylus_kv_store::StylusKVStore;

    #[test]
    fn test_set_and_get() {
        let mut kv = StylusKVStore::new();
        kv.set("key1".to_string(), "value1".to_string());
        assert_eq!(kv.get(&"key1".to_string()), Some(&"value1".to_string()));
    }

    #[test]
    fn test_overwrite_value() {
        let mut kv = StylusKVStore::new();
        kv.set("key1".to_string(), "value1".to_string());
        kv.set("key1".to_string(), "new_value".to_string());
        assert_eq!(kv.get(&"key1".to_string()), Some(&"new_value".to_string()));
    }

    #[test]
    fn test_delete_key() {
        let mut kv = StylusKVStore::new();
        kv.set("key1".to_string(), "value1".to_string());
        assert_eq!(kv.delete(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(kv.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_key_not_found() {
        let kv = StylusKVStore::new();
        assert_eq!(kv.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_contains_key() {
        let mut kv = StylusKVStore::new();
        kv.set("key1".to_string(), "value1".to_string());
        assert!(kv.contains(&"key1".to_string()));
    }
}
