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
use crate::createsha;

const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";


pub fn install(rawpkg: &String, option: bool) -> Result<()> {
    File::create("/var/cache/tmp.raw").context("Not running as root, aborting")?;
   let path = if Path::new("/etc/raw.conf").exists() {
        let conf = fs::read_to_string("/etc/raw.conf").context("Failed to open raw.conf file")?;
        let source = conf
            .lines()
            .find(|l| l.starts_with("source="))
            .and_then(|l| l.split_once("source=").map(|(_, p)| p.to_string()));
        let root = conf
            .lines()
            .find(|l| l.starts_with("root="))
            .and_then(|l| l.split_once("root=").map(|(_, p)| p.to_string()));
        source.or(root).context("No source path or root path defined in raw.conf")?

    } else {
        println!("No need to check signature");
        "none".to_string()
    };

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
                        let pkgname = content.iter().find(|l| l.contains(".raw.")).context("Failed to find raw package")?;
                        println!("{}", pkgname);
                        if option == true {
                            install(pkgname, true)?;
                        }
                        env::set_current_dir(saved)?;
                        let depends: Vec<String> = depends(rawpkg);
                        for i in depends {
                            install(&i, false)?;
                        }
                        return Ok(());
                    }
                }
            }
        }
    }

    if option == false {
        eprintln!("Checking conflict for rawpkg: {:?}", rawpkg);
        if Path::new("/tmp/conflict").exists() {
            fs::remove_file("/tmp/conflict")?;
        } else {
            conflict(&rawpkg).context("Conflict checking failed")?;
        }
    }
    
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).context("Failed to get pkgname")?;
    if path != "none" {
        let hash = createsha(&rawpkg)?;
        if path.ends_with("/") {
            let path = path.rsplit_once("/").map(|(path, _)| path).context("Failed to get index.raw path");
        }
        let index = fs::read_to_string(format!("{}/index.raw", path)).context("Failed to open index.raw")?;
        let sha = index.lines().find(|l| l.contains(&format!("{}/Pkgfile", pkg))).context("Package not present in index.raw")?;
        let meta: Vec<&str> = sha.split("|").collect();
        let sha = meta.get(3).context("Failed to get signature")?.to_string();
        if sha != hash {
            anyhow::bail!("Failed to check signature, exit !")
        }
    }
    if Path::new(&format!("/tmp/{}", pkg)).exists() {
        env::set_current_dir(format!("/tmp/{}", pkg))?;
    } else {
        fs::create_dir(format!("/tmp/{}", pkg))?;
        env::set_current_dir(format!("/tmp/{}", pkg))?;
        extract(rawpkg).context("Didn't find the archive to unpack")?;
        println!("{}Conflict Detection might not have been executed be careful{}", RED, RESET);
        
    }
    env::set_current_dir(format!("/tmp/{}", pkg))?;
    let opts = match option {
        true => CopyOptions {
            overwrite: true,
            follow_symlinks: false,
            restrict_symlinks: false,
            content_only: false,
            ..Default::default()
        },
        false => CopyOptions {
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
        .context("Pre-installation failed")?;
        fs::remove_file(format!("{}.pre-install", pkg)).context("Unable to remove pre-installation file")?;
    } else {
        println!("No pre-installation required");
    }
    copy_recursive(Path::new("."), Path::new("/"), &opts).unwrap();
    println!("running ldconfig.....");
    Command::new("bash")
    .args(["-c", "ldconfig"])
    .status()
    .context("Failed to run ldconfig")?;
    let automatic = Path::new("automatic").exists();
    if Path::new(&format!("{}.post-install", pkg)).exists() {
        let post_install = format!("chmod u+x {}.post-install && ./{}.post-install", pkg, pkg);
        println!("Starting post-installation.");
        Command::new("bash")
        .args(["-c", &post_install])
        .status()
        .context("Failed to run post-install")?;
        fs::remove_file(format!("{}.post-install", pkg))?;
    } else {
        println!("No post-installation required");
    }
    fs::create_dir(format!("/var/lib/pkg/DB/{}", pkg)).context(format!("/var/lib/pkg/DB/{} already exists", pkg))?;
    if Path::new(&format!("/{}.pre-remove", pkg)).exists() {
        fs::copy(format!("/{}.pre-remove", pkg), format!("/var/lib/pkg/DB/{}/{}.pre-remove", pkg, pkg))?;
    }
    if Path::new(&format!("/{}.post-remove", pkg)).exists() {
        fs::copy(format!("/{}.post-remove", pkg), format!("/var/lib/pkg/DB/{}/{}.post-remove", pkg, pkg))?;
    }
    if automatic == true {
        fs::copy("/automatic", format!("/var/lib/pkg/DB/{}/automatic", pkg))?;
    }
    fs::copy("/META", format!("/var/lib/pkg/DB/{}/META", pkg))?;
    fs::copy(format!("/{}.footprint", pkg), format!("/var/lib/pkg/DB/{}/files", pkg))?;
    fs::remove_file("/META")?;
    fs::remove_file(format!("/{}.footprint", pkg))?;
    fs::remove_file(format!("/{}", rawpkg))?;
    if Path::new(&format!("/{}.pre-install", pkg)).exists() {
        fs::remove_file(format!("/{}.pre-install", pkg))?;
    }
    if Path::new(&format!("/{}.post-install", pkg)).exists() {
        fs::remove_file(format!("/{}.post-install", pkg))?;
    }
    let content = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", pkg))?;
    if content.contains(".desktop") {
        if Path::new("/usr/bin/gtk-update-icon-cache").exists() {
            Command::new("bash")
            .args(["-c", "glib-compile-schemas /usr/share/glib-2.0/schemas"])
            .status()
            .context("Failed to recompile schemas")?;
            println!("Compiling gschemas")
        }
        if Path::new("/usr/bin/gtk-update-icon-cache").exists() {
            for entry in WalkDir::new("/usr/share/icons").max_depth(1).min_depth(1) {
                let foot = entry.unwrap().path().display().to_string();
                if file_type(&foot) == false {
                    env::set_current_dir("/").unwrap();
                    env::set_current_dir(&foot).unwrap();
                    let directory = format!("/usr/bin/gtk-update-icon-cache -f -t {}", foot);
                    Command::new("bash")
                    .args(["-c", &directory])
                    .status()
                    .context("Failed to update icon cache")?;
                    println!("Updating icon cache");
                }
                
            }
        }
    }

    Ok(())
}
