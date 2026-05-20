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


use std::path::Path;
use std::fs;
use std::env;
use recursive_copy::{copy_recursive, CopyOptions};
use anyhow::{Result};
use anyhow::Context;
use std::fs::File;
use crate::extract::extract;


pub fn bootstrap(rawpkg: &String, bootstrap_path: &str) -> Result<()> {
    println!("\x1b[31;1m[WARN] Please use this function only to install base packages, it will not run any post installation nor ldconfig !\x1b[0m");
    //let pkg_name = rawpkg.split_once(".raw").map(|(name, _)| name).unwrap_or(rawpkg);
    File::create("/var/cache/tmp.raw").context("Not running as root, aborting")?;
    fs::remove_file("/var/cache/tmp.raw").unwrap();
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap();
    fs::create_dir_all(format!("{}/var/lib/pkg/DB/{}/", bootstrap_path, pkg)).unwrap();
    println!("Copying {} to /var/lib/pkg/DB/{}/ in bootstrap directory", rawpkg, pkg);
    fs::copy(rawpkg, format!("{}/var/lib/pkg/DB/{}/{}", bootstrap_path, pkg, rawpkg)).unwrap();
    env::set_current_dir(format!("{}/var/lib/pkg/DB/{}", bootstrap_path, pkg)).unwrap();
    if rawpkg.ends_with(".tar.gz") || rawpkg.ends_with(".tgz") {
        extract(rawpkg)?;
    } else {
        println!("No package in the format required : ABORTING");
        std::process::exit(1);
    }
    let opts = CopyOptions {
        overwrite: true,
        follow_symlinks: false,
        restrict_symlinks: false,
        content_only: false,
        ..Default::default()
    };
    copy_recursive(Path::new("."), Path::new(bootstrap_path), &opts).unwrap();
    fs::remove_dir_all(format!("{}/var/lib/pkg/DB/{}", bootstrap_path, pkg)).unwrap();
    fs::create_dir(format!("{}/var/lib/pkg/DB/{}", bootstrap_path, pkg)).unwrap();
    fs::copy(format!("{}/META", bootstrap_path), format!("{}/var/lib/pkg/DB/{}/META", bootstrap_path, pkg)).unwrap();
    fs::copy(format!("{}/{}.footprint", bootstrap_path, pkg), format!("{}/var/lib/pkg/DB/{}/files", bootstrap_path, pkg)).unwrap();
    fs::remove_file(format!("{}/META", bootstrap_path)).unwrap();
    fs::remove_file(format!("{}/{}.footprint", bootstrap_path, pkg)).unwrap();
    fs::remove_file(format!("{}/{}", bootstrap_path, rawpkg)).unwrap();
    //let content = line.lines();
    Ok(())
}