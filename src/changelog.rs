// Raw is a simple package manager written in rust, it aims to be compatible with the Pkgfiles written that works with pkgmk from pkgutils/cards
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.



use std::path::Path;
use std::fs;
use std::env;
use crate::download::download;
use std::collections::HashSet;
use anyhow::{Result, Context};

pub fn changelog() -> Result<()> {
    if Path::new("/etc/raw.conf").exists() {
        let checkmode = fs::read_to_string("/etc/raw.conf")?;
        if checkmode.contains("mode=binary") {
            if let Some(line) = checkmode.lines().find(|l| l.starts_with("url=")) {
                let url = line.split_once("url=").map(|(_, url)| url).context("Failed to fetch url")?;
                env::set_current_dir("/var/cache/").context("Failed to change directory to /var/cache/")?;
                let index = download(&format!("{}/index.raw", url))?;
                let mut packages = Vec::new();
                for e in fs::read_dir("/var/lib/pkg/DB/.").context("Failed to read /var/lib/pkg/DB/")?.filter_map(|e| e.ok()) {
                    let directory_tmp = e.file_name();
                    let directory = directory_tmp.to_str().context("Failed to get packages name")?;
                    packages.push(directory.to_string());
                }
                let set: HashSet<_> = packages.into_iter().collect();
                let mut upgrade = Vec::new();
                for s in index.lines() {
                    let name = s.split_once("/Pkgfile").map(|(name, _)| name).context("Failed to get name")?.rsplit_once("/").map(|(_, name)| name).context("Failed to get name")?;
                    if set.contains(name) {
                        let meta = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", name)).context(format!("Failed to read {} META file", name))?;
                        let version = meta.lines().find(|l| l.starts_with("V")).context("Failed to get version")?.split_once("V").map(|(_, version)| version).context("Failed to get version")?;
                        let release = meta.lines().find(|l| l.starts_with("r")).context("Failed to get release line")?.split_once("r").map(|(_, release)| release).context("Failed to get version name")?;
                        let meta: Vec<&str> = s.split("|").collect();
                        let distversion = meta.get(1).context("Failed to get distant version")?.to_string();
                        let distrelease = meta.get(2).context("Failed to get distant release")?.to_string();
                        if version != distversion || release != distrelease {
                            upgrade.push(name);
                        }
                    }
                    
                }
                if !upgrade.is_empty() {
                    let number = upgrade.len();
                    println!("Packages to update : {}", number);
                    println!("List of packages : {:?}", upgrade);

                } else {
                    println!("All packages are up to date");
                }
            } else {
                anyhow::bail!("No url set");
            }
        } else {
            anyhow::bail!("Mode binary not set");
        }
    } else {
        anyhow::bail!("/etc/raw.conf doesn't exist");
    }
    Ok(())
}