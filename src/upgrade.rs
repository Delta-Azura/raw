use std::fs;
use std::path::Path;
use crate::get::get;
use crate::download::download;
use std::env;
use crate::getconf::getconf;
use anyhow::{Result, Context};


pub fn upgrade() -> Result<()> {
    let (mode, trash, url) = getconf().unwrap();
    if mode != "binary" {
        println!("Raw isn't used in binary mode, cannot connect to the repo");
        std::process::exit(1);
    }
    env::set_current_dir("/var/cache/").unwrap();
    let metadata = download(&format!("{}/index.raw", url))?;
    let index_raw = fs::read_to_string(metadata).context("Download failed")?;
    for i in index_raw.lines() {
        let i = i.trim();
        if i.is_empty() { continue; }
        let pkg = i.split_once('_').map(|(pkg, _)| pkg).unwrap().split_once('/').map(|(_, pkg)| pkg).unwrap();
        let version = i.split_once('_').map(|(_, version)| version).unwrap().split_once('#').map(|(version, _)| version).unwrap();
        let release = i.split_once('#').map(|(_, release)| release).unwrap();
        if Path::new(&format!("/var/lib/pkg/DB/{}", pkg)).exists() {
            let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", pkg)).unwrap();
            let mut content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
            let version_i = content.iter().find(|l| l.starts_with('V')).unwrap().to_string().split_once('V').map(|(_, version)| version).unwrap().to_string();
            let release_i = content.iter().find(|r| r.starts_with('r')).unwrap().to_string().split_once('r').map(|(_, release)| release).unwrap().to_string();
            if format!("{}{}", version, release) != format!("{}{}", version_i, release_i) {
                get(pkg);
            } else {
                println!("Package already up to date");
            }
        } else {
            continue;
        }
    }
    Ok(())
}