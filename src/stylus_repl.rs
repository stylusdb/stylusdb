use log::error;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::stylus_kv_store::StylusKVStore;

pub fn start_stylus_repl() {
    env_logger::init();

    let mut kv = StylusKVStore::new();
    let mut rl = Editor::<()>::new();

    println!("Welcome to StylusDB REPL!");

    loop {
        let readline = rl.readline("STYLUSDB==> ");
        match readline {
            Ok(line) => {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();

                match parts.as_slice() {
                    ["GET", key] => match kv.get(&key.to_string()) {
                        Some(value) => println!("{} = {}", key, value),
                        None => println!("{} not found", key),
                    },
                    ["SET", key, value] => {
                        kv.set(key.to_string(), value.to_string());
                        println!("Set {} = {}", key, value);
                    }
                    ["DELETE", key] => match kv.delete(&key.to_string()) {
                        Some(value) => println!("Deleted {} = {}", key, value),
                        None => println!("{} not found", key),
                    },
                    ["CONTAINS", key] => {
                        if kv.contains(&key.to_string()) {
                            println!("Key {} exists in the store.", key);
                        } else {
                            println!("Key {} does not exist in the store.", key);
                        }
                    }
                    ["QUIT"] | ["EXIT"] => {
                        println!("Goodbye!");
                        break;
                    }
                    _ => println!("Unknown command! Try again."),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                error!("Error: {:?}", err);
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
