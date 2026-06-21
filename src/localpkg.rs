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