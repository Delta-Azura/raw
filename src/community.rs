use std::env;
use std::path::Path;
use std::fs;
use anyhow::{Context, Result};
use crate::package;

pub fn community(pkg: &str) -> Result<()> {
    if !Path::new("/etc/raw.conf").exists() {
        anyhow::bail!("/etc/raw.conf doesn't exist");
    }
    let conf = fs::read_to_string("/etc/raw.conf")?;
    if conf.lines().any(|l| l.starts_with("community=")) {
        let mut url = conf.lines().find(|l| l.starts_with("community=")).context("Failed to read community line")?.split_once("community=").map(|(_, community)| community).context("Failed to get community git")?;
        if url.ends_with("/") {
            url = url.split_once("/").map(|(url, _)| url).context("Failed to adapt url")?;
        }
        let url = format!("{}/{}", url, pkg);
        let mut opt = git2::FetchOptions::new();
        opt.depth(1);
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(opt);
        let home = env::var("HOME")?;
        env::set_current_dir(home)?;
        if Path::new(pkg).exists() {
            fs::remove_dir_all(pkg)?;
        }
        match builder.clone(&url, Path::new("/var/cache/state")) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
        env::set_current_dir(pkg)?;
        package(None)?;
    } else {
        anyhow::bail!("Community setting isn't initialized in raw.conf");
    }
    Ok(())
}