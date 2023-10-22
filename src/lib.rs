use std::collections::HashMap;
use std::io;
use std::io::Write;

pub struct KVStore {
    store: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> Self {
        KVStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.store.insert(key, value)
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &String) -> Option<String> {
        self.store.remove(key)
    }

    pub fn contains(&self, key: &String) -> bool {
        self.store.contains_key(key)
    }
}

fn main() {
    let mut kv = KVStore::new();

    // Print a welcome message when the REPL starts
    println!("############################### Welcome to KVStore REPL! ###############################");

    loop {
        let mut input = String::new();

        // Prompt for user input
        print!("STYLUSDB==> ");
        io::stdout().flush().expect("Failed to flush");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["GET", key] => {
                match kv.get(&key.to_string()) {
                    Some(value) => println!("{} = {}", key, value),
                    None => println!("{} not found", key),
                }
            },
            ["SET", key, value] => {
                kv.set(key.to_string(), value.to_string());
                println!("Set {} = {}", key, value);
            },
            ["DELETE", key] => {
                match kv.delete(&key.to_string()) {
                    Some(value) => println!("Deleted {} = {}", key, value),
                    None => println!("{} not found", key),
                }
            },
            ["CONTAINS", key] => {
                if kv.contains(&key.to_string()) {
                    println!("Key {} exists in the store.", key);
                } else {
                    println!("Key {} does not exist in the store.", key);
                }
            },
            ["QUIT"] | ["EXIT"] => {
                println!("........... Goodbye! ...........");
                break;
            },
            _ => println!("Unknown command! Try again."),
        }
    }
}
