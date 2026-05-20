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
use std::process::Command;
use std::env;
use recursive_copy::{copy_recursive, CopyOptions};
use crate::conflict::conflict;
use anyhow::{Result};
use anyhow::Context;
use std::fs::File;
use walkdir::WalkDir;
use crate::file_type::file_type;
use crate::getconf::getconf;
use crate::depends::depends;
use crate::extract::extract;



pub fn install(rawpkg: &String, option: Option<&str>) -> Result<()> {
    File::create("/var/cache/tmp.raw").context("Not running as root, aborting")?;
    fs::remove_file("/var/cache/tmp.raw")?;
    if !rawpkg.contains(".raw.") {
        let (mode, root, _trash) = getconf().unwrap();
        if mode != "source" {
            println!("Please use raw get or install an archive .raw");
            std::process::exit(1)
        } else {
            let saved = env::current_dir()?;
            env::set_current_dir(root)?;
            let index = fs::read_to_string("index.raw")?;
            if index.lines().any(|l| l.contains(&format!("/{}/", rawpkg))) {
                let path_to_pkgfile = index.lines().find(|l| l.contains(&format!("/{}/", rawpkg))).ok_or("Didn't find");
                let path = path_to_pkgfile.unwrap().split_once("/Pkgfile").map(|(path, _)| path).ok_or("Failed");
                env::set_current_dir(path.unwrap())?;
                let content: Vec<String> = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).filter_map(|e| e.file_name().into_string().ok()).collect();
                if content.iter().any(|f| f.contains(".raw.")) {
                    if content.iter().any(|f| f.contains(rawpkg)) {
                        let pkgname = content.iter().find(|l| l.contains(".raw.")).unwrap();
                        println!("{}", pkgname);
                        install(pkgname, None)?;
                        env::set_current_dir(saved)?;
                        let depends: Vec<String> = depends(rawpkg);
                        for i in depends {
                            install(&i, None)?;
                        }
                        return Ok(());
                    }
                }
            }
        }
    }
    eprintln!("DEBUG conflict rawpkg: {:?}", rawpkg);
    if Path::new("/tmp/conflict").exists() {
        fs::remove_file("/tmp/conflict")?;
    } else {
        if Some("-f") == option {
            println!("overwriting");
        } else {
            conflict(&rawpkg);
        }
    }
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap();
 
    if Path::new(&format!("/tmp/{}", pkg)).exists() {
        env::set_current_dir(format!("/tmp/{}", pkg))?;
    } else {
        fs::create_dir(format!("/tmp/{}", pkg))?;
        println!("1");
        env::set_current_dir(format!("/tmp/{}", pkg))?;
        println!("2");
        if rawpkg.ends_with(".tar.gz") || rawpkg.ends_with(".tgz") {
            extract(rawpkg)?;
        } else {
            println!("No package in the format required : ABORTING");
            std::process::exit(1);
        }
    }
    env::set_current_dir(format!("/tmp/{}", pkg))?;
    let opts = match option {
        Some("-f") => CopyOptions {
            overwrite: true,
            follow_symlinks: false,
            restrict_symlinks: false,
            content_only: false,
            ..Default::default()
        },
        _ => CopyOptions {
            overwrite: false,
            follow_symlinks: false,
            restrict_symlinks: false,
            content_only: false,
            ..Default::default()
        },
    };
    if Path::new(&format!("{}.pre-install", pkg)).exists() {
        let pre_install = format!("chmod u+x {}.pre-install && ./{}.pre-install", pkg, pkg);
        println!("Starting pre-installation.");
        Command::new("bash")
        .args(["-c", &pre_install])
        .status()
        .unwrap();
        fs::remove_file(format!("{}.pre-install", pkg))?;
    } else {
        println!("No pre-installation required");
    }
    copy_recursive(Path::new("."), Path::new("/"), &opts).unwrap();
    println!("running ldconfig.....");
    Command::new("bash")
    .args(["-c", "ldconfig"])
    .status()
    .unwrap();
    let automatic = Path::new("automatic").exists();
    if Path::new(&format!("{}.post-install", pkg)).exists() {
        let post_install = format!("chmod u+x {}.post-install && ./{}.post-install", pkg, pkg);
        println!("Starting post-installation.");
        Command::new("bash")
        .args(["-c", &post_install])
        .status()
        .unwrap();
        fs::remove_file(format!("{}.post-install", pkg))?;
    } else {
        println!("No post-installation required");
    }
    fs::create_dir(format!("/var/lib/pkg/DB/{}", pkg)).unwrap();
    if Path::new(&format!("/{}.pre-remove", pkg)).exists() {
        fs::copy(format!("/{}.pre-remove", pkg), format!("/var/lib/pkg/DB/{}/{}.pre-remove", pkg, pkg)).unwrap();
    }
    if Path::new(&format!("/{}.post-remove", pkg)).exists() {
        fs::copy(format!("/{}.post-remove", pkg), format!("/var/lib/pkg/DB/{}/{}.post-remove", pkg, pkg)).unwrap();
    }
    if automatic == true {
        fs::copy("/automatic", format!("/var/lib/pkg/DB/{}/automatic", pkg)).unwrap();
    }
    fs::copy("/META", format!("/var/lib/pkg/DB/{}/META", pkg)).unwrap();
    fs::copy(format!("/{}.footprint", pkg), format!("/var/lib/pkg/DB/{}/files", pkg)).unwrap();
    fs::remove_file("/META").unwrap();
    fs::remove_file(format!("/{}.footprint", pkg)).unwrap();
    fs::remove_file(format!("/{}", rawpkg)).unwrap();
    if Path::new(&format!("/{}.pre-install", pkg)).exists() {
        fs::remove_file(format!("/{}.pre-install", pkg)).unwrap();
    }
    if Path::new(&format!("/{}.post-install", pkg)).exists() {
        fs::remove_file(format!("/{}.post-install", pkg)).unwrap();
    }
    let content = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", pkg)).unwrap();
    //let content = line.lines();
    if content.contains(".desktop") {
        if Path::new("/usr/bin/gtk-update-icon-cache").exists() {
            Command::new("bash")
                // glib-compile-schemas /usr/share/glib-2.0/schemas
            .args(["-c", "glib-compile-schemas /usr/share/glib-2.0/schemas"])
            .status()
            .unwrap();
            println!("Compiling gschemas")
        }
        if Path::new("/usr/bin/gtk-update-icon-cache").exists() {
            for entry in WalkDir::new("/usr/share/icons").max_depth(1).min_depth(1) {
                let foot = entry.unwrap().path().display().to_string();
                if file_type(&foot) == false {
                    env::set_current_dir("/").unwrap();
                    env::set_current_dir(&foot).unwrap();
                    println!("{}", foot);
                    let directory = format!("/usr/bin/gtk-update-icon-cache -f -t {}", foot);
                    Command::new("bash")
                    .args(["-c", &directory])
                    .status()
                    .unwrap();
                    println!("Updating icon cache");
                }
                
            }
        }
    }

    Ok(())
}
