use std::fs;
use anyhow::{Result, Context};

pub fn localpkg(pkg: &str) -> Result<(bool, Vec<(String, String)>)> {
    let conf = fs::read_to_string("/etc/raw.conf").context("Failed to open raw.conf file")?;
    let mut localdata: Vec<(String, String)> = Vec::new(); 
    let mut localpkg = false;
    if conf.contains("local=") {
        let path = conf.lines().find(|l| l.starts_with("local=")).context("Failed to get local line in raw.conf")?.split_once("local=").map(|(_, line)| line).context("Failed to get path to local repo")?;
        if path == "true" {
            let path = conf.lines().find(|l| l.starts_with("root=")).context("Failed to get local line in raw.conf")?.split_once("root=").map(|(_, line)| line).context("Failed to get path to local repo")?;
            let index = fs::read_to_string(format!("{}/index.raw", path)).context("Failed to read index.raw please run a raw index before going any further")?;
            let path = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", pkg))).context("Failed to get matching line in index.raw")?;
            let path = path.split_once("/Pkgfile").map(|(path, _)| path).context("Failed to get local package path")?;
            let entry: Vec<String> = fs::read_dir(&path)?.filter_map(|e| e.ok()).filter_map(|e| e.file_name().into_string().ok()).collect();
            for i in entry {
                if i.contains(".raw.") {
                    let version = i.split_once(".").map(|(_, version)| version).context("Failed to get version")?.split_once("#").map(|(version, _)| version).context("Failed to get version")?;
                    let release = i.split_once("#").map(|(_, release)| release).context("Failed to get release")?.split_once(".").map(|(version, _)| version).context("Failed to get version")?;
                    localdata.push((version.to_string(), release.to_string()));
                    localpkg = true;
                } else {
                    continue;
                }
            }
        }
    }
    return Ok((localpkg, localdata))
}