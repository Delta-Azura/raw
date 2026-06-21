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
use goblin::elf::Elf;

pub fn get_needed_libs(path: &Path) -> Vec<String> {
    let buf = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return vec![],
    };
    if let Ok(elf) = Elf::parse(&buf) {
        return elf.libraries.iter().map(|s| s.to_string()).collect();
    }
    vec![]
}

pub fn scan_pkg_dir(pkg_dir: &Path) -> Vec<String> {
    let mut needed = std::collections::HashSet::new();
    for entry in walkdir::WalkDir::new(pkg_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let libs = get_needed_libs(entry.path());
            needed.extend(libs);
        }
    }
    needed.into_iter().collect()
}