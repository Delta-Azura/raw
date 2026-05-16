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
use std::env;
use std::fs;
use crate::download::download;
use crate::getconf;


pub fn search(pkg: &str) -> Result<(String)> {
    let (mode, path, url) = getconf().unwrap(); 
    if mode != "binary" {
        env::set_current_dir(path)?;
        let content = fs::read_to_string("index.raw")?.to_string();
        let file = content.lines();
        for e in file {
            if e.contains(pkg) {
                println!("Package found here : {}", e);
                return Ok(e.to_string())
            }
        }
    return Err(anyhow::anyhow!("Package not found"));
    } else {
        let index = download(&format!("{}/index.raw", url))?;
        let content = fs::read_to_string("index.raw")?.to_string();
        let file = content.lines();
        for e in file {
            if e.contains(pkg) {
                println!("Package found here : {}", e);
                return Ok(e.to_string())
            } 
        }
        return Err(anyhow::anyhow!("Package not found"));
    }
}