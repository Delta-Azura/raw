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
use std::fs;
use std::path::Path;
use std::env::current_dir;
use anyhow::Result;
use std::fs::File;
use std::process::Command;


pub fn remove(rawpkg: &String, option: bool) -> Result<()> {
    File::create("/var/cache/raw.tmp")?;
    fs::remove_file("/var/cache/raw.tmp")?;
    let current = current_dir()?;
    let check = format!("/var/lib/pkg/DB/{}", rawpkg);

    if Path::new(&check).exists() {
        if option == false {
            for e in fs::read_dir("/var/lib/pkg/DB/.").unwrap().filter_map(|e| e.ok()) {
                let directory = e.file_name();
                let directory = directory.to_str().unwrap();
                let meta = format!("/var/lib/pkg/DB/{}/META", directory);
                // checking corrupted packages
                if meta.contains(&format!("{}", rawpkg)) {
                    continue;
                }
                let meta = fs::read_to_string(meta)?;
                if meta.contains(rawpkg) {
                    anyhow::bail!("Impossible to remove this package as it's a necessary depend for {}", directory);
                }
            }
        }
        env::set_current_dir(format!("/var/lib/pkg/DB/{}", rawpkg))?;
        if Path::new(&format!("/var/lib/pkg/DB/{}/{}.pre-remove", rawpkg, rawpkg)).exists() {
            let pre_remove = format!("chmod u+x {}.pre-remove && ./{}.pre-remove", rawpkg, rawpkg);
            println!("Starting pre-removal.");
            Command::new("bash")
            .args(["-c", &pre_remove])
            .status()
            .unwrap();
        } else {
            println!("No pre-removal required");
        }
        let post_remove = match Path::new(&format!("/var/lib/pkg/DB/{}/{}.post-remove", rawpkg, rawpkg)).exists() {
            true => {
                fs::copy(format!("/var/lib/pkg/DB/{}/{}.post-remove", rawpkg, rawpkg), format!("/tmp/{}.post-remove", rawpkg)).unwrap();
                format!("chmod u+x {}.post-remove && ./{}.post-remove", rawpkg, rawpkg)
            }
            false => {
                println!("no post removal required");
                format!("no")
            }
        };
        let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", rawpkg))?;
        let content = file.lines();
        env::set_current_dir("/tmp")?;
        fs::remove_dir_all(format!("/var/lib/pkg/DB/{}", rawpkg))?;
        let protected = vec!["bin", "lib", "lib64", "sbin", "usr/share/info/dir"];
        for i in content {
            let to_remove = i.split_whitespace().next().unwrap();
            if !protected.contains(&to_remove) {
                if !to_remove.starts_with("etc") {
                    let _ = fs::remove_file(format!("/{}", to_remove));
                    let _ = fs::remove_dir(format!("/{}", to_remove));
                } else {
                    continue;
                }
            }
        }
        if post_remove != "no" {
            println!("Executing post-remove trigger");
            Command::new("bash")
            .args(["-c", &post_remove])
            .status()
            .unwrap();
        }
    } else {
            anyhow::bail!("This package isn't installed, can't remove it");
    }
    env::set_current_dir(current)?;
    println!("{} successfully removed", rawpkg);
    Ok(())
}
