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

use std::fs::File;
use std::fs;
use crate::remove;
use std::path::Path;
use crate::install::install;
use crate::conflict::conflict;
use anyhow::{Result};
use anyhow::Context;
use crate::verifysha;


pub fn update(rawpkg: &String) -> Result<()> {
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap().to_string();
    if Path::new(&format!("/var/lib/pkg/DB/{}", pkg)).exists() {
        File::create("/tmp/conflict").unwrap();
        println!("removing previous package");
        let path = fs::read_to_string("/etc/raw.conf")?;
        let path = path.lines().find(|l| l.starts_with("root=")).context("Failed to check for parent directory of index.raw")?.split_once("root=").map(|(_, path)| path).context("Failed to get path")?;
        verifysha("source", Some(path.to_string()), rawpkg)?;
        remove(&pkg, true)?;
        conflict(&rawpkg)?;
        println!("Installing the new one");
        install(&rawpkg, false, true)?;
        if Path::new("/tmp/conflict").exists() {
            fs::remove_file("/tmp/conflict").context("Unable to remove /tmp/conflict,")?;
        }
    } else {
        anyhow::bail!("Package isn't installed");
    }
    Ok(())

}