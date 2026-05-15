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



use anyhow::{Result, Context};
use std::fs;
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[0;32m";

pub fn orphans() -> Result<()> {
    let mut orphans_list: Vec<String> = Vec::new();
    let mut required = std::collections::HashSet::new();
    let packages: Vec<String> = fs::read_dir("/var/lib/pkg/DB").unwrap().filter_map(|e| e.ok()).filter_map(|e| e.file_name().into_string().ok()).collect();
    for i in &packages {
        let infos = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", i)).unwrap();
        let lines = infos.lines();
        for x in lines {
            if x.starts_with("R") {
                let deps = x.trim_start_matches('R').split_whitespace();
                for dep in deps {
                    required.insert(dep.to_string());
                }
            }
        }
    }
    for x in &packages {
        if !required.contains(x) {
            orphans_list.push(x.to_string());
        }
    }
    for x in &orphans_list {
        println!("{}", x);
    }
    println!("{}{} orphans found{}", GREEN, orphans_list.len(), RESET);
    Ok(())
}
