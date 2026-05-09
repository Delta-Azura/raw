use anyhow::{Result};
use std::env;
use std::fs;
use crate::download::download;
use crate::getconf;


pub fn search(pkg: &str) -> Result<()> {
    let (mode, path, url) = getconf().unwrap(); 
    if mode != "binary" {
        env::set_current_dir(path)?;
        let content = fs::read_to_string("index.raw")?.to_string();
        let file = content.lines();
        for e in file {
            if e.contains(pkg) {
                println!("Package found here : {}", e);
            }
        }
    } else {
        let index = download(&format!("{}/index.raw", url))?;
        let content = fs::read_to_string("index.raw")?.to_string();
        let file = content.lines();
        for e in file {
            if e.contains(pkg) {
                println!("Package found here : {}", e);
            }
        }  
    }
    Ok(())
}