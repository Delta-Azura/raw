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
use anyhow::{Result, Context};
use crate::download::download;
use crate::install::install;
use std::fs;
use std::env;
use crate::depends::depends;
use std::path::Path;
use crate::update::update;
use std::fs::File;



pub fn get(pkg: &str) -> Result<()> {
    let (mode, root, url) = getconf().unwrap();
    if mode != "binary" {
        println!("Raw isn't used in binary mode, cannot connect to the repo");
        std::process::exit(1);
    }
    env::set_current_dir("/var/cache/").unwrap();
    let metadata = download(&format!("{}/index.raw", url))?;
    let index_raw = fs::read_to_string(metadata).context("Download failed")?;
    if index_raw.contains(&format!("{}", pkg)) {
        let rawpkg = index_raw.lines().any(|l| l.contains(&format!("/{}/", pkg)));
        if rawpkg != true {
            println!("Package doesn't exists");
            std::process::exit(1)
        }
        let line = index_raw.lines().find(|l| l.contains(&format!("/{}/", pkg))).context("Failed to get pkgname")?;
        let version = line.split_once('_').map(|(_, version)| version).unwrap().split_once('#').map(|(version, _)| version).context("Failed to get package name")?;
        let release = line.split_once('#').map(|(_, release)| release).context("Failed to get package name")?;
        let collection = line.split_once('/').map(|(collection, _)| collection).context("Failed to get package name")?;
        // %23 = #
        let path = format!("{}/{}/{}/{}.{}%23{}.raw.tar.xz", url, collection, pkg, pkg, version, release);
        println!("{}", path);
        env::set_current_dir(root).context("Failed to change to the download directory")?;
        let tarball = download(&path)?;
        if Path::new(&format!("/var/lib/pkg/DB/{}", pkg)).exists() {
            update(&tarball)?; 
        } else {
            //fs::copy(tarball, )
            install(&tarball, None)?;
        }
        let dependencies = depends(pkg);
        for i in &dependencies {
            get(i)?;
            File::create(format!("/var/lib/pkg/DB/{}/automatic", i)).context("Failed to add automatic file for oprhans")?;
        }
    } else {
        println!("Not found");
        std::process::exit(1)
    }

    Ok(())

}
