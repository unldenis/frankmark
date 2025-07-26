mod config;
mod error;
mod generator;
mod models;
mod navigation;
mod parser;
mod template;
mod utils;

use std::env;

use crate::error::FrankmarkResult;
use crate::generator::generate_site;

fn run(folder_path: &str) -> FrankmarkResult<()> {
    generate_site(folder_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder_path = args.get(1).map(|s| s.as_str()).unwrap_or("demo");

    if let Err(e) = run(folder_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
