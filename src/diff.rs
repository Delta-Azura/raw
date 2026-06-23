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
use anyhow::{Result};
use anyhow::Context;
use crate::download::download;
use std::env;


pub fn diff(pkg: &str) -> Result<()> {
    let conf = fs::read_to_string("/etc/raw.conf").context("Raw.conf doesn't exist")?;
    if conf.contains("mode=source") {
        check_source(pkg, conf)?;
    } else {
        check_binary(pkg, conf)?;
    }
    Ok(())
}


pub fn check_binary(pkg: &str, conf: String) -> Result<()> {
    let mut url = String::new();
    let (currentver, currentrel) = parse_local_version(pkg)?;
    if conf.contains("local=true") {
        let result = check_local(pkg, &conf)?;
        if result == false {
            env::set_current_dir("/var/cache/").context("/var/cache/ doesn't exist, your system might be in pain")?;
            url = conf.lines().find(|l| l.starts_with("url=")).context("Failed to get url variable")?.split_once("url=").map(|(_, url)| url).context("Failed to get url, check your raw.conf file")?.to_string();
            if url.ends_with("/") {
                url = url.rsplit_once("/").map(|(url, _)| url).context("Failed to format url to prepare for index.raw download")?.to_string();
            }
            download(&format!("{}/index.raw", url))?;
            let index = fs::read_to_string("index.raw").context("There might be a problem with the download index.raw")?;
            let line = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", pkg))).context("This package isn't available in the repo")?;
            let meta: Vec<&str> = line.split("|").collect();
            let version = meta.get(1).context("Failed to get distant version")?.to_string();
            let release = meta.get(2).context("Failed to get distant release")?.to_string();
            if version != currentver || release != currentrel {
                println!("Version or release do not match.\nCurrent version and release = {}-{}.\nFound version and release = {}-{}", version, release, currentver, currentrel);
            } else {
                println!("This package is up to date");
            }
        }
    }
    Ok(())
}


pub fn check_local(pkg: &str, conf: &String) -> Result<bool> {
    let mut version = String::new();
    let mut release = String::new();
    let mut result = true;
    let root = conf.lines().find(|l| l.starts_with("root=")).context("Failed to get root= line")?.split_once("root=").map(|(_, path)| path).context("Root variable isn't available")?;
    println!("{}index.raw", root);
    let index = fs::read_to_string(&format!("{}/index.raw", root)).context("Please run raw index first")?;
    if index.lines().any(|l| l.contains(&format!("{}/Pkgfile", pkg))) {
        let path = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", pkg))).context("Failed to get path")?.split_once("/Pkgfile").map(|(path, _)| path).context("Failed")?;
        println!("{}/{}", root, path);
        let entries: Vec<String> = fs::read_dir(format!("{}/{}", root, path))?.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect();
        if !entries.iter().any(|e| e.contains(".raw.")) {
            anyhow::bail!("No package has been generated");
        } else {
            for i in entries {
                if !i.contains(".raw.") {
                    continue;
                } else {
                    version = i.split_once(".").map(|(_, version)| version).context("Failed to get version")?.split_once("#").map(|(version, _)| version).context("Failed to get version")?.to_string();
                    release = i.split_once("#").map(|(_, release)| release).context("Failed to get release")?.split_once(".raw").map(|(release, _)| release).context("Failed to get release")?.to_string();
                    break;
                }
            }
        }
        let (currentver, currentrel) = parse_local_version(pkg)?; 
        if currentver != version || currentrel != release {
            println!("Version or release do not match.\nCurrent version and release = {}-{}.\nFound version and release = {}-{}", version, release, currentver, currentrel);
        } else {
            println!("This package is up to date");
        }
    } else {
        result = false;
    }
    Ok(result)
}


pub fn check_source(pkg: &str, conf: String) -> Result<()> {
    let mut version = String::new();
    let mut release = String::new();
    let root = conf.lines().find(|l| l.starts_with("root=")).context("Failed to get root= line")?.split_once("root=").map(|(_, path)| path).context("Root variable isn't available")?;
    println!("{}index.raw", root);
    let index = fs::read_to_string(&format!("{}/index.raw", root)).context("Please run raw index first")?;
    if index.lines().any(|l| l.contains(&format!("{}/Pkgfile", pkg))) {
        let path = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", pkg))).context("Failed to get path")?.split_once("/Pkgfile").map(|(path, _)| path).context("Failed")?;
        println!("{}/{}", root, path);
        let entries: Vec<String> = fs::read_dir(format!("{}/{}", root, path))?.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect();
        if !entries.iter().any(|e| e.contains(".raw.")) {
            anyhow::bail!("No package has been generated");
        } else {
            for i in entries {
                if !i.contains(".raw.") {
                    continue;
                } else {
                    version = i.split_once(".").map(|(_, version)| version).context("Failed to get version")?.split_once("#").map(|(version, _)| version).context("Failed to get version")?.to_string();
                    release = i.split_once("#").map(|(_, release)| release).context("Failed to get release")?.split_once(".raw").map(|(release, _)| release).context("Failed to get release")?.to_string();
                    break;
                }
            }
        }
        let (currentver, currentrel) = parse_local_version(pkg)?; 
        if currentver != version || currentrel != release {
            println!("Version or release do not match.\nCurrent version and release = {}-{}.\nFound version and release = {}-{}", version, release, currentver, currentrel);
        } else {
            println!("This package is up to date");
        }
    }
    Ok(())
}

pub fn parse_local_version(pkg: &str) -> Result<(String, String)> {
    let mut currentver = String::new();
    let mut currentrel = String::new();
    if Path::new(&format!("/var/lib/pkg/DB/{}", pkg)).exists() {
            let meta = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", pkg)).context("This package informations are not available")?;
            currentver = meta.lines().find(|l| l.starts_with("V")).context("Failed to get current version")?.split_once("V").map(|(_, version)| version).context("Failed to get current version")?.to_string();
            currentrel = meta.lines().find(|l| l.starts_with("r")).context("Fauled to get current release")?.split_once("r").map(|(_, release)| release).context("Failed to get current release")?.to_string();
    } else {
        anyhow::bail!("This package isn't installed");
    }
    return Ok((currentver, currentrel));
}