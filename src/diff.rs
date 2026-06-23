use std::path::Path;
use std::fs;
use anyhow::{Result};
use anyhow::Context;

pub fn diff(pkg: &str) -> Result<()> {
    let mut version = String::new();
    let mut release = String::new();
    let mut currentver = String::new();
    let mut currentrel = String::new();
    let conf = fs::read_to_string("/etc/raw.conf").context("Raw.conf doesn't exist")?;
    if conf.contains("mode=source") {
        let root = conf.lines().find(|l| l.starts_with("root=")).context("Failed to get root= line")?.split_once("root=").map(|(_, path)| path).context("Root variable isn't available")?;
        println!("{}index.raw", root);
        let index = fs::read_to_string(&format!("{}/index.raw", root)).context("Please run raw index first")?;
        if index.lines().any(|l| l.contains(&format!("{}/Pkgfile", pkg))) {
            let path = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", pkg))).context("Failed to get path")?.split_once("/Pkgfile").map(|(path, _)| path).context("Failed")?;
            println!("{}/{}", root, path);
            let entries: Vec<String> = fs::read_dir(format!("{}/{}", root, path))?.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect();
            if !entries.iter().any(|e| e.contains(".raw.")) {
                anyhow::bail!("No package has been generated");
            } else {
                for i in entries {
                    if !i.contains(".raw.") {
                        continue;
                    } else {
                        version = i.split_once(".").map(|(_, version)| version).context("Failed to get version")?.split_once("#").map(|(version, _)| version).context("Failed to get version")?.to_string();
                        release = i.split_once("#").map(|(_, release)| release).context("Failed to get release")?.split_once(".raw").map(|(release, _)| release).context("Failed to get release")?.to_string();
                        break;
                    }
                }
            }
            if Path::new(&format!("/var/lib/pkg/DB/{}", pkg)).exists() {
                let meta = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", pkg)).context("This package informations are not available")?;
                currentver = meta.lines().find(|l| l.starts_with("V")).context("Failed to get current version")?.split_once("V").map(|(_, version)| version).context("Failed to get current version")?.to_string();
                currentrel = meta.lines().find(|l| l.starts_with("r")).context("Fauled to get current release")?.split_once("r").map(|(_, release)| release).context("Failed to get current release")?.to_string();
            } else {
                anyhow::bail!("This package isn't installed");
            }
            if currentver != version || currentrel != release {
                    println!("Version or release do not match.\nCurrent version and release = {}-{}.\nFound version and release = {}-{}", version, release, currentver, currentrel);
            } else {
                println!("This package is up to date");
            }
        }
    }
    Ok(())
}