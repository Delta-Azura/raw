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
use std::env;
use anyhow::{Result};
use anyhow::Context;
use std::path::Path;



pub fn info(rawpkg: &String) -> Result<()> {
    env::set_current_dir(format!("/var/lib/pkg/DB/{}", rawpkg)).context("Package isn't installed")?;

    let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", rawpkg)).unwrap();
    // Adding vect to be able to read properly, without this it would be unable to read if the order isn't respected
    let content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    let name = content.iter().find(|l| l.starts_with('N')).context("Failed to get pkgname")?.split_once('N').map(|(_, name)| name).context("Failed to get pkgname")?.to_string();
    println!("Name : {}", name);
    let version = content.iter().find(|l| l.starts_with('V')).context("Failed to get pkgver")?.to_string().split_once('V').map(|(_, version)| version).context("Failed to get pkgver")?.to_string();
    println!("Version = {}", version);
    let description = content.iter().find(|l| l.starts_with('D')).context("Failed to get description")?.to_string().split_once('D').map(|(_, description)| description).context("Failed to get description")?.to_string();
    println!("Description = {}", description);
    let packager = content.iter().find(|l| l.starts_with('P')).context("Failed to get packager name")?.to_string().split_once('P').map(|(_, packager)| packager).context("Failed to get packager name")?.to_string();
    println!("Packager = {}", packager);
    let collection = content.iter().find(|l| l.starts_with('c')).context("Failed to get collection")?.to_string().split_once('c').map(|(_, collection)| collection).context("Failed to get collection")?.to_string();
    println!("Collection = {}", collection);
    if Path::new(&format!("/var/lib/pkg/DB/{}/automatic", rawpkg)).exists() {
        println!("Manual Installation : NO");
    } else {
        println!("Manual Installation : YES");
    }
    Ok(()) 
}