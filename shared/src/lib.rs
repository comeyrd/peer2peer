pub mod config;           // Declares the module (from config.rs)
pub use config::*;        // Re-exports everything from config

pub fn hello_world() {
    println!("Hello world from shared!");
}
