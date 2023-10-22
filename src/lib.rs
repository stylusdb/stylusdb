/*###############################################################
###                                                             #
###                        KVStore Code                         #
###                                                             #
#################################################################
*/

use std::collections::HashMap;
use std::io;

// Define a public struct named KVStore to represent a key-value store.
// NOTE: This code is responsible for a simple REPL (Read-Eval-Print Loop) 
//that supports basic commands like SET, GET, DELETE, CONTAINS, and QUIT.

pub struct KVStore {
    // The store field is a HashMap that associates strings (keys) with strings (values).
    store: HashMap<String, String>,
}

// Implement methods for the KVStore struct.
impl KVStore {
    // Constructor for KVStore, creating a new instance with an empty HashMap.
    pub fn new() -> Self {
        KVStore {
            store: HashMap::new(),
        }
    }

    // Insert a key-value pair into the store, returning the previous value if the key already existed.
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.store.insert(key, value)
    }

    // Retrieve the value associated with a key, returning an Option that can be Some(value) or None.
    pub fn get(&self, key: &String) -> Option<&String> {
        self.store.get(key)
    }

    // Delete a key-value pair by key, returning the value if the key existed.
    pub fn delete(&mut self, key: &String) -> Option<String> {
        self.store.remove(key)
    }

    // Check if a key exists in the store, returning a boolean.
    pub fn contains(&self, key: &String) -> bool {
        self.store.contains_key(key)
    }
}

// Entry point of the program.
fn main() {
    // Create a mutable instance of the KVStore.
    let mut kv = KVStore::new();
    
    // Enter a loop to continuously read and process user input.
    loop {
        // Create a mutable string to store user input.
        let mut input = String::new();

        // Read a line of input from the user.
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Split the input into whitespace-separated parts and collect them into a vector.
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        // Match the command based on the input parts.
        match parts.as_slice() {
            // If the command is "GET", retrieve and print the associated value.
            ["GET", key] => {
                match kv.get(&key.to_string()) {
                    Some(value) => println!("{} = {}", key, value),
                    None => println!("{} not found", key),
                }
            },
            // If the command is "SET", insert a key-value pair into the store and print a message.
            ["SET", key, value] => {
                kv.set(key.to_string(), value.to_string());
                println!("Set {} = {}", key, value);
            },
            // If the command is "DELETE", remove a key-value pair and print a message.
            ["DELETE", key] => {
                match kv.delete(&key.to_string()) {
                    Some(value) => println!("Deleted {} = {}", key, value),
                    None => println!("{} not found", key),
                }
            },
            // If the command is "CONTAINS", check if a key exists and print a message.
            ["CONTAINS", key] => {
                if kv.contains(&key.to_string()) {
                    println!("Key {} exists in the store.", key);
                } else {
                    println!("Key {} does not exist in the store.", key);
                }
            },
            // If the command is "QUIT" or "EXIT", exit the loop and end the program.
            ["QUIT"] | ["EXIT"] => break,
            // If the command is unknown, print an error message.
            _ => println!("Unknown command! Try again."),
        }
    }
}
