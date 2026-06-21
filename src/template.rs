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

use std::fs::OpenOptions;
use anyhow::{Result, Context};
use std::io::Write;
use std::fs::File;
use std::env;
use std::fs;
use crate::getconf;
use std::path::Path;


pub fn template(pkg: &str) -> Result<()> {
    match File::create("/var/cache/raw.tmp") {
        Ok(_) => {
            println!("You are building as root !");
            fs::remove_file("/var/cache/raw.tmp")?;
            std::process::exit(1)
        }
        Err(_e) => {}
    }
    let pwd = env::current_dir()?;
    let (mode, root, _trash) = getconf().unwrap();
    if mode != "source" {
        println!("Creating in binary mode");
    } else {
        if pwd.to_string_lossy().to_string() != root {
            println!("You are not creating the template in your directory set in your raw.conf, it will not work with raw index and raw build");
            env::set_current_dir(pwd).context("Failed to change directory")?;
        }
    }
    if Path::new(pkg).exists() {
        fs::remove_dir_all(pkg)?;
    }
    fs::create_dir(pkg).context("Needs to be root, cannot initiate as packages needs to be built as non-root")?;
    env::set_current_dir(pkg).context("Invalid directory")?;
    File::create("Pkgfile").context("Failed to create PKgfile")?;
    let mut pkgfile = OpenOptions::new().append(true).write(true).open("Pkgfile")?;
    let content_pkgfile = "description=\nname=\nrelease=\nversion=\nmakedepends=\nrundepends=\nsource=\nbuild() {\n\n}\n";
    writeln!(pkgfile, "{}", content_pkgfile).context("Failed to write pkgfile")?;
    Ok(())
}



