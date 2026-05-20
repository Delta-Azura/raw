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
use anyhow::Context;
use std::path::Path;
use crate::get::get;
use crate::install;


pub fn depends(pkg: &str) -> Vec<String> {  
    let mut stack = vec![pkg.to_string()];
    let mut visited = std::collections::HashSet::new();
    while let Some(rawpkg) = stack.pop() {
        if !visited.insert(rawpkg.clone()) {
            continue;
        }
        println!("{}", rawpkg);
        if !Path::new(&format!("/var/lib/pkg/DB/{}/META", rawpkg)).exists() {
            println!("{} isn't installed", rawpkg);
            let configuration = fs::read_to_string("/etc/raw.conf").context("Raw.conf doesn't exist").unwrap();
            if configuration.contains("mode source") {
                let _ = install(&rawpkg, None);
            } else {
                let _ = get(&rawpkg);
            }
            //std::process::exit(1)
        }
        let META = std::fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", rawpkg)).unwrap();
        let meta = META.lines().find(|l| l.starts_with("R")).unwrap().split_once('R').map(|(_, meta)| meta).unwrap();
        let dependencies : Vec<&str> =  meta.split_whitespace().collect();
        for i in dependencies.iter() {
            if !visited.contains(*i) {
                println!("{}", i);
                stack.push(i.to_string());
            }

        }

    }

    return stack
}
