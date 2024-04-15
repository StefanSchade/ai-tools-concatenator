mod config;
mod file_ops;
mod utils;
mod traversal;

use std::env;
use std::io;

use config::Config;
use traversal::concatenate_dir;
use file_ops::{initialize_output_file, read_gitignore};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_arguments(&args)?;
    let mut output = initialize_output_file(&config.output_file)?;

    let gitignore_patterns = read_gitignore(&config.source_dir)?;
    let always_exclude = config.default_excludes();

    concatenate_dir(
        &config.source_dir,
        &config.source_dir,
        &mut output,
        0,
        &gitignore_patterns,
        &config.exclude_suffixes,
        &always_exclude,
    )?;

    Ok(())
}
