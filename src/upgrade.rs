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
use std::path::Path;
use crate::get::get;
use crate::download::download;
use std::env;
use crate::getconf::getconf;
use crate::remove::remove;
use crate::install::install;
use crate::localpkg::localpkg;
use anyhow::{Result, Context};


pub fn upgrade() -> Result<()> {
    let (mode, _trash, url) = getconf().unwrap();
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
        let pkg = i.split_once("/Pkgfile").map(|(pkg, _)| pkg).context("Failed to get package name")?.rsplit_once("/").map(|(_, name)| name).context("Failed to get package name")?;
        let meta: Vec<&str> = i.split("|").collect();
        let version = meta.get(1).context("Failed to get version")?;
        let release = meta.get(2).context("Failed to get release")?;
        if Path::new(&format!("/var/lib/pkg/DB/{}", pkg)).exists() {
            let (localpkg, localdata) = localpkg(pkg)?;
            if localpkg == true {
                let (localver, localrel) = &localdata[0];
                if localver != version || localrel != release {
                    remove(&pkg.to_string(), true)?;
                    install(&pkg.to_string(), false)?;     
                }
            } else {
                continue;
            }
            let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", pkg)).unwrap();
            let content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
            let version_i = content.iter().find(|l| l.starts_with('V')).unwrap().to_string().split_once('V').map(|(_, version)| version).unwrap().to_string();
            let release_i = content.iter().find(|r| r.starts_with('r')).unwrap().to_string().split_once('r').map(|(_, release)| release).unwrap().to_string();
            if format!("{}{}", version, release) != format!("{}{}", version_i, release_i) {
                remove(&pkg.to_string(), true)?;
                get(pkg)?;
            } else {
                println!("Package already up to date");
            }
        } else {
            continue;
        }
    }
    Ok(())
}