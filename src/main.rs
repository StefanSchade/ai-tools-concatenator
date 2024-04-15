mod config;
mod file_ops;
mod utils;
mod traversal;

use config::Config;
use traversal::concatenate_dir;


use std::env;
use std::fs::OpenOptions;
use std::io::{self};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(config: &Config) -> io::Result<()> {
    let mut output = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&config.output_file)?;

    concatenate_dir(config, &config.source_dir, &mut output, 0)
}
