use anyhow::{Result};
use anyhow::Context;
use std::fs;
use crate::createsha;


pub fn verifysha(mode: &str, path: Option<String>, package: &str) -> Result<()> {
    //let mut index = String::new();
    //let Some(path) = path;
    let index = if mode == "binary" {
        fs::read_to_string("/var/cache/index.raw").context("Index.raw not present, corrupted or impossible to open")?
    } else {
        let path = path.context("No path provided to read index.raw")?;
        fs::read_to_string(format!("{}/index.raw", path)).context("Index.raw not present, corrupted or impossible to open")?
    };
    let sha = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", package))).context("Package not present in index.raw")?;
    let meta: Vec<&str> = sha.split("|").collect();
    let sha = meta.get(3).context("Failed to get package checksum")?.to_string();
    let hash = createsha(package)?; 
    if sha != hash {
        anyhow::bail!("Checksums don't match, exiting")
    }

    return Ok(())
}