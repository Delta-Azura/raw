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

use crate::getconf::getconf;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;
use anyhow::{Context, Result};
use std::path::Path;
use std::fs;
use crate::createsha;


pub fn index() -> Result <()> {
    if let Ok((_mode, path, _trash)) = getconf() {
        let path = if !path.ends_with("/") {
            format!("{}/", path)
        } else {
            path
        };
        if Path::new("index.raw").exists() {
            fs::remove_file("index.raw")?;
        }
        let mut rawfile = File::create("index.raw").context("This directory isn't usable as non-root, aborting")?;
        for entry in WalkDir::new(&path.trim()).min_depth(2) {
            let entries = entry.unwrap().path().display().to_string().split_once(&path.trim()).map(|(_, entries)| entries).unwrap().to_string();
            let (version, release, sha) = if entries.contains("Pkgfile") {
                let pkgfile = fs::read_to_string(&format!("{}{}", path, entries)).context("Pkgfile not found")?;
                let content: Vec<String> = pkgfile.lines().map(|l| l.to_string()).collect();
                let version = content.iter().find(|version| version.starts_with("version")).unwrap_or(&"version=unknown".to_string()).to_string().split_once("version=").map(|(_, version)| version).unwrap().to_string();
                let release = content.iter().find(|release| release.starts_with("release")).unwrap_or(&"release=1".to_string()).to_string().split_once("release=").map(|(_, version)| version).unwrap().to_string();
                let path = entries.split_once("/Pkgfile").map(|(path, _)| path).context("Failed to get path")?;
                let mut sha = "none".to_string();
                for check in fs::read_dir(path)? {
                    let checkraw = check?.file_name().to_string_lossy().to_string();
                    if checkraw.contains(".raw.") {
                        let checkraw = format!("{}/{}", path, checkraw);
                        sha = createsha(&checkraw)?
                    }
                }
                (version, release, sha)
            } else {
                continue;
            };
            let version = if version.contains("\"") {
                version.split_once("\"").map(|(_, version)| version).unwrap().split_once("\"").map(|(version, _)| version).unwrap()
            } else {
                &version
            };
            let release = if release.contains("\"") {
                release.split_once("\"").map(|(_, release)| release).unwrap().split_once("\"").map(|(release, _)| release).unwrap()
            } else {
                &release
            };
            writeln!(rawfile, "{}", &format!("{}|{}|{}|{}", entries, version, release, sha))?;
        }
    }
    Ok(())
}