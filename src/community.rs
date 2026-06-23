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
use std::path::Path;
use std::fs;
use anyhow::{Context, Result};
use crate::package;

pub fn community(pkg: &str) -> Result<()> {
    if !Path::new("/etc/raw.conf").exists() {
        anyhow::bail!("/etc/raw.conf doesn't exist");
    }
    let conf = fs::read_to_string("/etc/raw.conf")?;
    if conf.lines().any(|l| l.starts_with("community=")) {
        let mut url = conf.lines().find(|l| l.starts_with("community=")).context("Failed to read community line")?.split_once("community=").map(|(_, community)| community).context("Failed to get community git")?;
        if url.ends_with("/") {
            url = url.rsplit_once("/").map(|(url, _)| url).context("Failed to adapt url")?;
        }
        let url = format!("{}/{}", url, pkg);
        let mut opt = git2::FetchOptions::new();
        opt.depth(1);
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(opt);
        let home = env::var("HOME")?;
        env::set_current_dir(home)?;
        if Path::new(pkg).exists() {
            fs::remove_dir_all(pkg)?;
        }
        if Path::new(pkg).exists() {
            fs::remove_dir_all(pkg).context(format!("Failed to remove {} in the home user directory", pkg))?;
        }
        match builder.clone(&url, Path::new(pkg)) {
            Ok(repo) => repo,
            Err(e) => anyhow::bail!("failed to clone: {}", e),
        };
        env::set_current_dir(pkg)?;
        package(None)?;
    } else {
        anyhow::bail!("Community setting isn't initialized in raw.conf");
    }
    Ok(())
}