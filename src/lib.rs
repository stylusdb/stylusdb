pub mod stylus_kv_store;
mod stylus_repl;

pub use stylus_repl::start_stylus_repl;

fn main() {
    start_stylus_repl();
}