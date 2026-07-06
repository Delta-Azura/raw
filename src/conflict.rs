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
use crate::query;
use std::env;
use std::path::Path;
use crate::file_type::file_type;
use std::thread;
use std::time::Duration;
use crate::extract::extract;
use anyhow::{Result, Context};
use std::collections::HashSet;

pub fn conflict(rawpkg: &String) -> Result<()> {
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap().to_string();
    if Path::new(&format!("/tmp/{}", pkg)).exists() {
        fs::remove_dir_all(format!("/tmp/{}", pkg)).context("Failed to remove /tmp/pkg")?;
        fs::create_dir(format!("/tmp/{}", pkg)).context("Failed to create /tmp/pkg")?;
    } else {
        fs::create_dir(format!("/tmp/{}", pkg)).context("Failed to create /tmp/pkg")?;
    }
    fs::copy(rawpkg, format!("/tmp/{}/{}", pkg, rawpkg)).context("failed to copy to /tmp/")?;
    env::set_current_dir(format!("/tmp/{}", pkg)).context("Failed to channge directory to /tmp/pkgname")?;
    extract(rawpkg)?;
    let compare = fs::read_to_string(format!("/tmp/{}/{}.footprint", pkg, pkg)).context("Failed to read footprint")?;
    let compare_set: HashSet<&str> = compare.lines().filter_map(|l| l.split_whitespace().next()).collect();
    //let compare = binding.split_whitespace().next().unwrap();
    for e in fs::read_dir("/var/lib/pkg/DB/.").unwrap().filter_map(|e| e.ok()) {
        let directory_tmp = e.file_name();
        let directory = directory_tmp.to_str().unwrap();
        let files_path = format!("/var/lib/pkg/DB/{}/files", directory);
        // checking corrupted packages
        if !Path::new(&files_path).exists() { 
            println!("\x1b[31mPlease be careful {} is corrupted\x1b[0m", files_path);
            thread::sleep(Duration::from_secs(10));
            continue; 
        }
        let target = fs::read_to_string(&files_path).context("Failed to read package list of files")?;//.split_whitespace().next().unwrap();
        //let target = temp.split_whitespace().next().unwrap();
        for lines in target.lines() {
            let lines = lines.split_whitespace().next().unwrap_or("");
            //let release = variables.next().unwrap();
            if compare_set.contains(lines) {                    
                let list = format!("{}", lines);
                if list.is_empty() { continue; }
                //file_type(&list);
                if file_type(&list) == true {
                    let test = format!("/{}", &list);
                    if test != "/usr/share/info/dir" {
                        if !test.starts_with("/etc") {
                            if file_type(&test) == true {
                                let test = test.split_once("/").map(|(_, test)| test).context("Failed to parse conflict search")?;
                                let _owner = query(&test.to_string());
                                anyhow::bail!("");
                            }
                        }

                    }
                    
                }
                
            }
        }
    }
// File conflict
    for i in compare.lines() {
        let i = i.split_whitespace().next().unwrap_or("");
        let r = format!("/{}", i);
        if file_type(&r) == true {
            if r != "/usr/share/info/dir" {
                if !r.starts_with("/etc")  {
                    if Path::new(&r).exists() {
                        anyhow::bail!("File {} already present on the system", i);
                    }
                }
            }
        }
    }
    env::set_current_dir(format!("/tmp/{}", pkg)).context("Failed to set current dir to /tmp/pkgname")?;
    Ok(())
}