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


pub fn getconf() ->  Result<(String, String, String), String> {
    match Path::new("/etc/raw.conf").exists() {
        true => {
            let config = fs::read_to_string("/etc/raw.conf").unwrap();
            if config.clone().contains("mode=binary") {
                //.lines() important to only take the line concerned
                let repo = config.lines().find(|c| c.starts_with("source=")).unwrap().split_once("source=").map(|(_, repo)| repo).unwrap().to_string();
                let url = config.lines().find(|l| l.starts_with("url=")).unwrap().split_once("url=").map(|(_, repo)| repo).unwrap().to_string();
                println!("Url set as : {}", url);
                println!("Repo set as : {}", repo);
                env::set_current_dir(&repo.trim()).unwrap();
                return Ok(("binary".to_string(), repo, url));
            }
            if config.clone().contains("mode=source") {
                let root = config.lines().find(|c| c.starts_with("root=")).unwrap().split_once("root=").map(|(_, root)| root).unwrap().to_string();
                println!("Root directory set as : {}", root);
                env::set_current_dir(&root.trim()).unwrap();
                return Ok(("source".to_string(), root.to_string(), String::new()));
            } else {
                return Err("not specified".to_string());
            }
        }
        false => {
            println!("/etc/raw.conf file not found");
            std::process::exit(1)
        }
        
    }
}