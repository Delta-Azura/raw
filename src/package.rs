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

use std::fs::File;
use std::io::Write;
//use crate::Path;
use std::path::Path;
//use crate::fs;
use std::fs;
use std::fs::OpenOptions;
//use crate::env;
use std::env;
use std::process::Command;
use crate::download::download;
use crate::extract::extract;
use walkdir::WalkDir;
use anyhow::{Result, Context};
use crate::getconf::getconf;
use crate::get::get;
use crate::build::build;
use flate2::write::GzEncoder;
use flate2::Compression;

const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[0;32m";
const YELLOW: &str = "\x1b[33m";
use std::io::Read;

#[derive(Debug, PartialEq, Eq)]
pub enum ArchiveType {
    Zip,
    SevenZip,
    Rar,
    Gzip,
    Bzip2,
    Tar,
    Xz,
    Zstd,
    Unknown,
}

struct Signature {
    archive_type: ArchiveType, 
    magic: &'static [u8],
    offset: u64,
}


// list of signatures
static SIGNATURES: &[Signature] = &[
    Signature { archive_type: ArchiveType::Zip, magic: &[0x50, 0x4B, 0x03, 0x04], offset: 0 },
    Signature { archive_type: ArchiveType::Zip, magic: &[0x50, 0x4B, 0x05, 0x06], offset: 0 },
    Signature { archive_type: ArchiveType::SevenZip, magic: &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C], offset: 0 },
    Signature { archive_type: ArchiveType::Rar, magic: &[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07], offset: 0 },
    Signature { archive_type: ArchiveType::Gzip, magic: &[0x1F, 0x8B], offset: 0 },
    Signature { archive_type: ArchiveType::Bzip2, magic: &[0x42, 0x5A, 0x68], offset: 0 },
    Signature { archive_type: ArchiveType::Xz, magic: &[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00], offset: 0 },
    Signature { archive_type: ArchiveType::Zstd, magic: &[0x28, 0xB5, 0x2F, 0xFD], offset: 0 },
    // TAR magic number starts at offset 257
    Signature { archive_type: ArchiveType::Tar, magic: &[0x75, 0x73, 0x74, 0x61, 0x72], offset: 257 },
];


pub fn package(option: Option<&str>) -> Result<()> {
        match File::create("/var/cache/raw.tmp") {
        Ok(_) => {
            println!("You are building as root !");
            fs::remove_file("/var/cache/raw.tmp")?;
            std::process::exit(1)
        }
        Err(_e) => {}
    }
    match fs::exists("Pkgfile") {
        Ok(true) => println!("Starting to build"),
        Ok(false) => {
            println!("Pkgfile doesn't exist.");
            std::process::exit(1);
        }
        Err(e) => {
            println!("Error : {e}");
            std::process::exit(1);
        }
    }
    let output = Command::new("bash")
        .args(["-c", "set -e && source Pkgfile && echo $version && echo $name && echo $packager && echo $release && echo $description && echo $rundepends && echo ${source[@]} && echo ${makedepends[@]}"])
        .output()
        .unwrap_or_else(|e| {
            println!("{} [!] : ERROR CHECK THE PKGFILE : {} {}", RED, e, RESET);
            std::process::exit(1)
        });
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut variables  = stdout.lines();
    let version = variables.next().context("The pkgfile may be missing something, check out for the version")?;
    let name = variables.next().context("The name or something else might be incorrect")?; 
    let packager = variables.next().context("Packager name or something else might be broken")?;
    let release = variables.next().context("Release or something else might be broken")?;
    let description = variables.next().context("Description or something else might not be valid")?;
    let depends = variables.next().context("Depends might not be correct, check your pkgfile")?;
    let source = variables.next().context("The source might not be correct, check your pkgfile")?;
    let makedepends: Vec<String> = variables.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();    //if makedepends == "none" {

    let actual = std::env::current_dir().unwrap();
    let col = actual.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();
    let collection = std::env::current_dir().unwrap();
    let _current = collection.file_name().unwrap().to_str().unwrap().to_string();
    let collection = collection.display().to_string();
    println!("Setting collection as : {}", col);
    let mut meta = File::create("META").unwrap();
    let metadata = format!("N{}\nV{}\nr{}\nc{}\nD{}\nP{}\nR{}\n", name, version, release, col, description, packager, depends);
    write!(meta, "{}", metadata).unwrap();
    if Path::new("work").exists() {
        println!("Removing work/");
        fs::remove_dir_all("work/")?;
    }
    if Path::new("pkg").exists() {
        println!("Removing pkg/");
        fs::remove_dir_all("pkg/")?;
    }
    fs::create_dir("work")?;
    fs::create_dir("pkg")?;
 
    if !makedepends.is_empty() {
        println!("{}Checking for makedepends: {:?}{}", YELLOW, makedepends, RESET);
        for i in &makedepends {
            if Path::new(&format!("/var/lib/pkg/DB/{}", i)).exists() {
                println!("{}{} is installed{}", GREEN, i, RESET)
            } else {
                let (mode, trash, _url) = getconf().unwrap();
                if mode != "source" {
                    get(&i)?;
                }
                if mode == "source" {
                    env::set_current_dir(trash).context("Failed")?;
                    let index_raw = fs::read_to_string("index.raw").context("Index.raw doesn't exists, run raw index to create it")?;
                    let test = format!("/{}/", i);
                    let found = index_raw.lines().find(|line| line.contains(&test));
                    let chrp = found.unwrap().split_once("Pkgfile").map(|(chrp, _)| chrp).unwrap();
                    env::set_current_dir(format!("{}", chrp)).unwrap();
                    //let mut path_automatic = path_automatic.split_once("/Pkgfile").map(|(path_automatic, _)| path_automatic).unwrap();
                    let collection = std::env::current_dir().unwrap();
                    let _current = collection.file_name().unwrap().to_str().unwrap().to_string();
                    let collection = collection.display().to_string();
                    //let mut path_automatic = path_automatic.lines();
                    //let mut path_automatic = path_automatic.find(|l| l.contains(&format!("{}", i))).unwrap().split_once("Package found here : ").map(|(_, path)| path).unwrap().split_once("/Pkgfile").map(|(path_automatic, _)| path_automatic).unwrap();
                    println!("{}", collection);


                    File::create("automatic").context("Failed to create the automatic file, be careful while removing orphans")?;
                    
                    build(&i, Some("-y"))?;
                }
                
            }
        }
    }
    let building = format!("{}/work", collection);
    env::set_current_dir(&collection).unwrap();
    println!("Switching to the work directory {}", building);

    for src in source.split_whitespace() {
        if src.contains("http") {
            env::set_current_dir(&building)?;
            let tarball = download(src)?;
            env::set_current_dir(&collection)?;
            if tarball.contains(".patch.gz") {
                continue;
            } else {
                env::set_current_dir(&building)?;
                extract(&tarball)?;
                env::set_current_dir(&collection)?;
            }
        } else {
            env::set_current_dir(&collection)?;
            fs::copy(src, format!("work/{}", src))?;
            env::set_current_dir(&building)?;
            if src.contains(".patch.gz") {
                continue;
            } else {
                let mut file = File::open(src)?;
                let mut buffer = [0u8; 512];
                let bytes_read = file.read(&mut buffer)?;
                for sig in SIGNATURES {
                    let start = sig.offset as usize;
                    let end = start + sig.magic.len();
                    if bytes_read >= end && &buffer[start..end] == sig.magic {
                        println!("{} {}", src, collection);
                        env::set_current_dir(&building)?;
                        extract(&src.to_string())?;
                        env::set_current_dir(&collection)?;
                    }
                }
            }
        }
    }
    env::set_current_dir(&collection)?;
    let prepare = fs::read_to_string("Pkgfile").unwrap();
    let cmd = match (prepare.contains("prepare()"), prepare.contains("package()"), prepare.contains("build()")) {
        (true, true, true) => {
            format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && build && cd $SRC && package'")
        }
        (true, false, true) => {
            format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && build'")
        }
        (false, true, true) => {
            format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && build && cd $SRC && package'")
        }
        (false, false, true) => {
            format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && build'")
        }
        (true, true, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = build_style.split_once("=").map(|(_, style)| style).unwrap_or_else(|| {
                        println!("Invalid build= line");
                        std::process::exit(1)
                    });

                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/{} && cd $SRC && package'", style)
                    } else {
                        println!("No build style available for {}", build_style);
                        std::process::exit(1)
                    }
                }   
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/build-default && cd $SRC && package'")
                    } else {
                        println!("No default build style set in /etc/raw.d/build-default : aborting");
                        std::process::exit(1)
                    }
   
                }
            }
        }
        (true, false, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = build_style.split_once("=").map(|(_, style)| style).unwrap_or_else(|| {
                        println!("Invalid build= line");
                        std::process::exit(1)
                    });

                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/{}'", style)
                    } else {
                        println!("No build style available for {}", build_style);
                        std::process::exit(1)
                    }
                }   
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/build-default'")      
                    } else {
                        println!("No default build style set in /etc/raw.d/build-default : aborting");
                        std::process::exit(1)
                    }
   
                }
            }
        }
        (false, true, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = build_style.split_once("=").map(|(_, style)| style).unwrap_or_else(|| {
                        println!("Invalid build= line");
                        std::process::exit(1)
                    });
                 
                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/{} && cd $SRC && package'", style)
                    } else {
                        println!("No build style available for {}", build_style);
                        std::process::exit(1)
                    }
                }   
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/build-default && cd $SRC && package'")
                    } else {
                        println!("No default build style set");
                        std::process::exit(1)
                    }  
                    
   
                }
            }
        }
        (false, false, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = build_style.split_once("=").map(|(_, style)| style).unwrap_or_else(|| {
                        println!("Invalid build= line");
                        std::process::exit(1)
                    });

                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/{}'", style)
                    } else {
                        println!("No build style available for {}", build_style);
                        std::process::exit(1)
                    }
                }  
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/build-default'")
                    } else {
                        println!("No default build style available");
                        std::process::exit(1)
                    }
                }
            } 
        }
    };
    if !Path::new(&format!("{}/.local/share/raw/", env::var("HOME").unwrap())).exists() {
        fs::create_dir_all(format!("{}/.local/share/raw/", env::var("HOME").unwrap())).unwrap()
    }
    let log_path = format!("{}/.local/share/raw/raw.log", env::var("HOME").unwrap());
    if Path::new(&log_path).exists() {
        fs::remove_file(&log_path).unwrap();
        File::create(&log_path).context("Failed to create log file")?;
    } else {
        File::create(&log_path).context("Failed to  create log file")?;
    }
    let cmd = format!("{} 2>&1 | tee -a {}/.local/share/raw/raw.log", cmd, env::var("HOME").unwrap());
    let _output_build = match Command::new("bash")
    .args(["-c", &cmd])
    .env("MAKEFLAGS", format!("-j{}", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)))
    .env("CFLAGS", "-O2 -pipe")
    .env("CXXFLAGS", "-O2 -pipe")
    .status() {
            // need if s.success because of the type of answer from status
    Ok(s) if s.success() => {
        let log_file = format!("{} Build succeded {}", GREEN, RESET);
        println!("{} Build succeded {}", GREEN, RESET);
        let mut logfile = OpenOptions::new()
        .append(true)
        .write(true)
        .open(format!("{}/.local/share/raw/raw.log", env::var("HOME").unwrap()))
        .context("Failed to open log file")?;
        writeln!(logfile, "{:#?}", log_file)?;
        env::set_current_dir(&collection)?;
        fs::remove_dir_all("work").unwrap();
    }
    Ok(s) => {
            // Don't ask
        println!("{} [!] : ERROR : The build failed (code {:?}) {}", RED, s.code(), RESET);
        let log_file = format!("{} [!] : ERROR : The build failed (code {:?}) {}", RED, s.code(), RESET);
        let mut logfile = OpenOptions::new()
        .append(true)
        .write(true)
        .open(format!("{}/.local/share/raw/raw.log", env::var("HOME").unwrap()))
        .context("Failed to open log file")?;
        writeln!(logfile, "{:#?}", log_file)?;
        std::process::exit(1);
    }
    Err(e) => {
        let log_file = format!("{} [!] : ERROR The build failed {} {}", RED, e, RESET);
        println!("{} [!] : ERROR The build failed {} {}", RED, e, RESET);
        let mut logfile = OpenOptions::new()
        .append(true)
        .write(true)
        .open(format!("{}/.local/share/raw/raw.log", env::var("HOME").unwrap()))
        .context("Failed to open log file")?;
        writeln!(logfile, "{:#?}", log_file)?;
        std::process::exit(1);
    }
    };
    let prepare = format!("{}/pkg", collection);

    if Path::new(&format!("{}.{}#raw.tar.gz", name, version)).exists() {
        println!("Removing the previous generated package");
        fs::remove_file(format!("{}.{}#raw.tar.gz", name, version))?;
    }
    println!("Generating footprint and looking for changes");
    if Path::new(&format!("{}.footprint", name)).exists() {
        let existing = fs::read_to_string(format!("{}.footprint", name)).unwrap();
        fs::remove_file(format!("{}.footprint", name))?;
        let mut footprint = File::create(format!("{}.footprint", name)).unwrap();
        for entry in WalkDir::new(&prepare).follow_links(false) {
            let entry = entry?;
            let foot = entry.path().display().to_string();
            let pathpkg = foot.split_once(&prepare).map(|(_,pathpkg)| pathpkg).context("Not found")?.to_string();
            if pathpkg.is_empty() { continue; }
                let _list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
                if entry.file_type().is_symlink() {
                    let list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
                    let link = fs::read_link(entry.path())?;
                    writeln!(footprint, "{} -> {}", list, link.display())?;
                } else {
                    let list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
                writeln!(footprint, "{}", list)?;
            }

        }
        let footprint = fs::read_to_string(format!("{}.footprint", name)).unwrap();
        if existing == footprint {
            println!("Same")
        } else {
            for line in existing.lines() {
                if !footprint.lines().any(|l| l == line) {
                    println!("{} MISSING : {} {}", RED, line, RESET);
                }
            }
            for line in footprint.lines() {
                if !existing.lines().any(|l| l == line) {
                    println!("{} NEW : {} {}", GREEN, line, RESET);
                }
            }

        }
    } else {
        let mut footprint = File::create(format!("{}.footprint", name)).unwrap();
            for entry in WalkDir::new(&prepare).follow_links(false) {
                let entry = entry?;
                let foot = entry.path().display().to_string();
                let pathpkg = foot.split_once(&prepare).map(|(_,pathpkg)| pathpkg).context("Not found")?.to_string();
                if pathpkg.is_empty() { continue; }
                    let _list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
                    if entry.file_type().is_symlink() {
                        let list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
                        let link = fs::read_link(entry.path())?;
                        writeln!(footprint, "{} -> {}", list, link.display())?;
                    } else {
                        let list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
                        writeln!(footprint, "{}", list)?;
                    }
            }
    }

    fs::copy("META", "pkg/META").unwrap();
    fs::remove_file("META").unwrap();
    fs::copy(format!("{}.footprint", name), format!("pkg/{}.footprint", name)).unwrap();
    if Path::new("automatic").exists() {
        fs::copy("automatic", "pkg/automatic")?;
    }
    if Path::new(&format!("{}/{}.pre-install", collection, name)).exists() {
        fs::copy(format!("{}.pre-install", name), format!("pkg/{}.pre-install", name)).unwrap();
    } else {
        println!("No need to prepare pre-installation");
    }
    if Path::new(&format!("{}/{}.post-install", collection, name)).exists() {
        fs::copy(format!("{}.post-install", name), format!("pkg/{}.post-install", name))?;
    } else {
        println!("No need to prepare post-installation");
    }
    if Path::new(&format!("{}/{}.pre-remove", collection, name)).exists() {
        fs::copy(format!("{}.pre-remove", name), format!("pkg/{}.pre-remove", name))?;
    } else {
        println!("No need to prepare pre-remove");
    }
    if Path::new(&format!("{}/{}.post-remove", collection, name)).exists() {
        fs::copy(format!("{}.post-remove", name), format!("pkg/{}.post-remove", name))?;
    } else {
        println!("No need to prepare post-remove");
    }
    if option == Some("--clean") {
        println!("{}Removing makedepends{}", YELLOW, RESET);
        for i in &makedepends {
            Command::new("sudo")
                .args(["raw", "remove", &i])
                .status()?;
        }
    }
    println!("Generating package");
    let tar = File::create(format!("{}.{}#1.raw.tar.gz", name, version))?;
    let enc = GzEncoder::new(tar, Compression::default());
    let mut a = tar::Builder::new(enc);
    a.follow_symlinks(false);
    a.append_dir_all("", "pkg/")?;
    a.finish().unwrap();
    fs::remove_dir_all("pkg")?;
    Ok(())
}