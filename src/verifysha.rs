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
    let mut pkg = package;
    if pkg.contains(".raw.") {
        pkg = package.split_once(".").map(|(pkg, _)| pkg).context("Failed to isolated package name")?;
    }
    let sha = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", pkg))).context("Package not present in index.raw")?;
    let meta: Vec<&str> = sha.split("|").collect();
    let sha = meta.get(3).context("Failed to get package checksum")?.to_string();
    let hash = createsha(package)?; 
    if sha != hash {
        anyhow::bail!("Checksums don't match, exiting")
    }
    return Ok(())
}