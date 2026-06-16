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

use sha2::{Sha256, Digest};
use anyhow::{Result, Context};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn createsha(package: &str) -> Result<String> {
    let file = File::open(package).context("Failed to open newly generated archive")?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    let hash = hex::encode(hasher.finalize());
    println!("{}", hash);
    return Ok(hash);
}