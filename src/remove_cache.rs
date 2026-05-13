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
use std::fs;
use anyhow::Context;
use crate::file_type::file_type;

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[0;32m";
const YELLOW: &str = "\x1b[33m";


pub fn remove_cache() -> Result<()> {
    let cache: Vec<String> = fs::read_dir("/var/lib/pkg/")
    .unwrap()
    .filter_map(|e| e.ok())
    .filter_map(|e| e.file_name().into_string().ok())
    .collect();
    for i in cache {
        let full_path = format!("/var/lib/pkg/{}", i);
        if file_type(&full_path) == true {
            println!("{}", i);
            fs::remove_file(&full_path)?;
            println!("{}Successfully removed {}{}", GREEN, i, RESET);
        } else {
            println!("{}{} is not a package to remove, continue....{}", YELLOW, i, RESET);
            continue
        }
    }
    Ok(())
}