use anyhow::{Result};
use std::fs;
use anyhow::Context;
use crate::file_type::file_type;

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[0;32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[1;31m";


pub fn remove_cache() -> Result<()> {
    let cache: Vec<String> = fs::read_dir("/var/lib/pkg/")
    .unwrap()
    .filter_map(|e| e.ok())
    .filter_map(|e| e.file_name().into_string().ok())
    .collect();
    for i in cache {
        let full_path = format!("/var/lib/pkg/{}", i);
        if file_type(&full_path) == true {
            println!("{}", i);
            fs::remove_file(&full_path)?;
            println!("{}Successfully removed {}{}", RED, i, RESET);
        } else {
            println!("{}{} is not a package to remove, continue....{}", YELLOW, i, RESET);
            continue
        }
    }
    Ok(())
}