use std::fs;
use std::fs::File;
use std::io::{self};
use std::path::{Path};



use crate::config::Config;
use crate::file_ops::read_and_write_file;
use crate::utils;

pub fn concatenate_dir(
    config: &Config,
    current_path: &Path,
    file_handle: &mut File,
    depth: usize,
) -> io::Result<()> {
    if current_path.is_dir() {
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();

            if utils::is_excluded(&path, config) {
                continue;
            }

            if path.is_dir() {
                concatenate_dir(config, &path, file_handle, depth + 1)?;
            } else {
                read_and_write_file(config, file_handle, &path, depth)?;
            }
        }
    }
    Ok(())
}