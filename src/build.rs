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

use std::env;
use anyhow::{Result, Context};
use crate::package;
use std::fs;
use crate::getconf;
use std::process::Command;
use std::path::Path;

pub fn build(pkg: &str) -> Result<()> {
    getconf().unwrap();
    //env::set_current_dir(&pkg)?;
    let path = fs::read_to_string("index.raw").context("index.raw doesn't exist, please run raw index")?;
    let path = path.lines().find(|l| l.contains(&format!("{}/", pkg))).context("This package doesn't exists on the index")?.split_once("/Pkgfile").map(|(path, _)| path).unwrap().to_string();
    println!("{}", path);
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        if entry.file_name().to_string_lossy().contains(".raw.") {
            env::set_current_dir(&path)?;
            println!("current path is {}", path);
            let potential_package = entry.file_name().to_string_lossy().to_string();
            let pkgver =  entry.file_name().to_string_lossy().split_once('.').map(|(_, pkgver)| pkgver).unwrap().split_once("#").map(|(pkgver, _)| pkgver).unwrap().to_string();
            let pkgrel = entry.file_name().to_string_lossy().split_once('#').map(|(_, pkgver)| pkgver).unwrap().split_once(".").map(|(pkgver, _)| pkgver).unwrap().to_string();
            if !Path::new("Pkgfile").exists() {
                Command::new("sudo").args(["raw", "install", &potential_package]).status()?;
            } else {
                let pkgfile_comp = fs::read_to_string("Pkgfile")?;
                let pkgverfile = pkgfile_comp.lines().find(|l| l.starts_with("version=")).context("No line found")?.split_once("version=").map(|(_, version)| version).context("no pkg version mentionned in pkgfile")?;
                let pkgrelfile = pkgfile_comp.lines().find(|l| l.starts_with("release=")).context("No line found")?.split_once("release=").map(|(_, version)| version).context("no pkg release mentionned in pkgfile")?;
                if pkgver == pkgverfile || pkgrel == pkgrelfile {
                    Command::new("sudo").args(["raw", "install", &potential_package]).status()?;
                } else {
                    package(None)?;
                    Command::new("sudo").args(["raw", "install", &potential_package]).status()?;
                }
            }

        }
    }
    Ok(())
}