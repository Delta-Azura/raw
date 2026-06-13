use sha2::{Sha256, Digest};
use anyhow::{Result, Context};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn createsha(package: String) -> Result<()> {
    let file = File::open(package).context("Failed to open newly generated archive")?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    let hash = hex::encode(hasher.finalize());
    println!("{}", hash);
    Ok(())
}