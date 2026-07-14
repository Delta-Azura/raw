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
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::env;
use std::process::Command;
use crate::download::download;
use crate::extract::extract;
use walkdir::WalkDir;
use anyhow::{Result, Context, bail};
use crate::getconf::getconf;
use crate::get::get;
use flate2::write::GzEncoder;
use flate2::Compression;
use crate::download::download_parallel;
use tokio::task::JoinSet;
use crate::createsha;

use crate::getlibs::scan_pkg_dir;
use crate::query;

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

pub struct Signature {
    archive_type: ArchiveType, 
    magic: &'static [u8],
    offset: u64,
}


// list of signatures
pub static SIGNATURES: &[Signature] = &[
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
    match fs::exists("Pkgfile") {
        Ok(true) => println!("Starting to build"),
        Ok(false) => {
            bail!("Pkgfile doesn't exist.");
        }
        Err(e) => {
            bail!("Error : {e}");
        }
    }
    let output = Command::new("bash")
        .args(["-c", "set -e && source Pkgfile && echo $version && echo $name && echo $packager && echo $release && echo $description && echo $rundepends && echo ${source[@]} && echo ${makedepends[@]}"])
        .output()
        .unwrap_or_else(|e| {
            println!("{} [!] : ERROR CHECK THE PKGFILE : {} {}", RED, e, RESET);
            //critical thing to correct
            std::process::exit(1)
        });
    let stdout = String::from_utf8(output.stdout).context("Failed to get variables from Pkgfile")?;
    let mut variables  = stdout.lines();
    let version = variables.next().context("The pkgfile may be missing something, check out for the version")?;
    let name = variables.next().context("The name or something else might be incorrect")?; 
    let packager = variables.next().context("Packager name or something else might be broken")?;
    let release = variables.next().context("Release or something else might be broken")?;
    let description = variables.next().context("Description or something else might not be valid")?;
    let depends = variables.next().context("Depends might not be correct, check your pkgfile")?;
    let source = variables.next().context("The source might not be correct, check your pkgfile")?;
    let makedepends: Vec<String> = variables.next().context("Failed to get makedepends")?.split_whitespace().map(|s| s.to_string()).collect();    //if makedepends == "none" {
    let pkgfile = fs::read_to_string("Pkgfile").context("Package file doesn't exist")?;
    let mut keep_sources = true;
    for i in pkgfile.lines() {
        if i.starts_with("RAW_KEEP_SOURCES=false") {
            keep_sources = false;
            break;
        } else {
            keep_sources = true
        };
    }
    let actual = std::env::current_dir().context("Failed to get current dir")?;
    let col = actual.parent().context("Failed to get current dir")?.file_name().context("Failed to get current dir")?.to_str().context("Failed to get current dir")?.to_string();
    let collection = std::env::current_dir().context("Failed to get current dir")?;
    let _current = collection.file_name().context("Failed to get current dir")?.to_str().context("Failed to get current dir")?.to_string();
    let collection = collection.display().to_string();
    println!("Setting collection as : {}", col);
    let mut meta = File::create("META").context("Failed to create META file")?;
    let metadata = format!("N{}\nV{}\nr{}\nc{}\nD{}\nP{}\nR{}\n", name, version, release, col, description, packager, depends);
    write!(meta, "{}", metadata).context("Failed to write metadata")?;
    if Path::new("work").exists() {
        println!("Removing work/");
        fs::remove_dir_all("work/").context("Failed to remove existing workdir")?;
    }
    if Path::new("pkg").exists() {
        println!("Removing pkg/");
        fs::remove_dir_all("pkg/").context("Failed to remove existing pkg directory")?;
    }
    fs::create_dir("work").context("Failed to create work directory")?;
    fs::create_dir("pkg").context("Failed to create pkg directory")?;
 
    if !makedepends.is_empty() {
        println!("{}Checking for makedepends: {:?}{}", YELLOW, makedepends, RESET);
        for i in &makedepends {
            if Path::new(&format!("/var/lib/pkg/DB/{}", i)).exists() {
                println!("{}{} is installed{}", GREEN, i, RESET)
            } else {
                let Ok((mode, trash, _url)) = getconf() else {
                    anyhow::bail!("Failed to get current configuration for raw");
                };
                if mode != "source" {
                    get(&i).context("Failed to get makedepends")?;
                }
                if mode == "source" {
                    env::set_current_dir(trash).context("Failed")?;
                    let index_raw = fs::read_to_string("index.raw").context("Index.raw doesn't exists, run raw index to create it")?;
                    let test = format!("/{}/", i);
                    let found = index_raw.lines().find(|line| line.contains(&test));
                    let chrp = found.context("Failed to get correct path for selected makedepends")?.split_once("Pkgfile").map(|(chrp, _)| chrp).context("Failed to get correct path for selected makedepends")?;
                    env::set_current_dir(format!("{}", chrp)).context("Failed to enter makedepends directory")?;
                    let collection = std::env::current_dir().unwrap();
                    let _current = collection.file_name().unwrap().to_str().unwrap().to_string();
                    let collection = collection.display().to_string();

                    File::create("automatic").context("Failed to create the automatic file, be careful while removing orphans")?;
                    for entry in fs::read_dir(collection)? {
                        let entry = entry?;
                        if entry.file_name().to_string_lossy().contains(".raw.") {
                            let pkgver =  entry.file_name().to_string_lossy().split_once('.').map(|(_, pkgver)| pkgver).context("Failed to get package release")?.split_once("#").map(|(pkgver, _)| pkgver).context("Failed to get package release")?.to_string();
                            let pkgrel = entry.file_name().to_string_lossy().split_once('#').map(|(_, pkgver)| pkgver).context("Failed to get package version")?.split_once(".").map(|(pkgver, _)| pkgver).context("Failed to get package version")?.to_string();
                            if !Path::new("Pkgfile").exists() {
                                Command::new("sudo").args(["raw", "install", &i]).output().context("Failed to install makedepend")?;
                            } else {
                                let pkgfile_comp = fs::read_to_string("Pkgfile")?;
                                let pkgverfile = pkgfile_comp.lines().find(|l| l.starts_with("version=")).context("No line found")?.split_once("version=").map(|(_, version)| version).context("no pkg version mentionned in pkgfile")?;
                                let pkgrelfile = pkgfile_comp.lines().find(|l| l.starts_with("release=")).context("No line found")?.split_once("release=").map(|(_, version)| version).context("no pkg release mentionned in pkgfile")?;
                                if pkgver == pkgverfile || pkgrel == pkgrelfile {
                                    Command::new("sudo").args(["raw", "install", &i]).output().context("Failed to install makedepends")?;
                                } else {
                                    package(None)?;
                                    Command::new("sudo").args(["raw", "install", &i]).output().context("Failed to install makedepends")?;
                                }
                            }
                        }
                    }
                    
                }
                
            }
        }
    }
    let building = format!("{}/work", collection);
    env::set_current_dir(&collection).context("Failed to get in the correct directory")?;
    println!("{}", collection);
    //println!("Switching to the work directory {}", building);
    if source.split_whitespace().count() > 1 {
        for src in source.split_whitespace() {
            if !src.contains("http") {
                if !src.contains(".patch.gz") {
                    println!("{}Checking the sources{}", YELLOW, RESET);
                    let mut file = File::open(&src)?;
                    let mut buffer = [0u8; 512];
                    let bytes_read = file.read(&mut buffer)?;
                    let is_archive = is_archive(bytes_read, &buffer)?;
                    if is_archive == true {
                        env::set_current_dir(&building)?;
                        extract(&src.to_string())?;
                        env::set_current_dir(&collection)?;
                    } else {
                        fs::copy(src, format!("work/{}", src))?;
                    }
                } else {
                    fs::copy(src, format!("work/{}", src))?;
                }
            }
        }
        env::set_current_dir(&collection)?;
        let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                let mut set = JoinSet::new();
                for src in source.split_whitespace() {
                    if src.contains("http") {
                        let src = src.to_string();
                        set.spawn(async move {
                            download_parallel(&src).await
                        });
                    }
                }
                while let Some(result) = set.join_next().await {
                    let tarball = result??;
                    if !tarball.contains(".patch.gz")  {
                        println!("{}Checking the sources{}", YELLOW, RESET);
                        let mut file = File::open(&tarball)?;
                        let mut buffer = [0u8; 512];
                        let bytes_read = file.read(&mut buffer)?;
                        let is_archive = is_archive(bytes_read, &buffer)?;
                        if is_archive == true {
                            println!("{} {}", tarball, collection);
                            fs::copy(&tarball, format!("{}/{}", building, tarball))?;
                            env::set_current_dir(&building)?;
                            extract(&tarball.to_string())?;
                            env::set_current_dir(&collection)?;
                            if keep_sources != true {
                                fs::remove_file(&tarball).context("Unable to remove the downloaded archive")?;
                            } else {
                                println!("Skipping removal of the sources");
                            }
                        } else {
                            fs::copy(&tarball, format!("{}/{}", building, tarball))?;
                            if keep_sources != true {
                                fs::remove_file(tarball).context("Failed to remove source file")?;
                            }
                        }
                    } else {
                        fs::copy(&tarball, format!("work/{}", tarball))?;
                        if kee_sources != true {
                            fs::remove_file(tarball).context("Failed to remove downloaded tarball")?;
                        }
                    }
                }
                Ok::<(), anyhow::Error>(())
            })?;

    } else {
        let src = source.trim();
        if src.contains("http") {
            env::set_current_dir(&collection)?;
            let tarball = download(src)?;
            env::set_current_dir(&collection)?;
            if !tarball.contains(".patch.gz") {
                fs::copy(&tarball, format!("work/{}", tarball))?;
                env::set_current_dir(&building)?;
                extract(&tarball)?;
                env::set_current_dir(&collection)?;
                if keep_sources != true {
                    fs::remove_file(&tarball)?;
                }
            } else {
                fs::copy(&tarball, format!("work/{}", tarball))?;
                if keep_sources != true {
                    fs::remove_file(tarball)?;
                }
            }
        } else {
            env::set_current_dir(&collection)?;
            fs::copy(src, format!("work/{}", src))?;
            env::set_current_dir(&building)?;
            if !src.contains(".patch.gz") {
                println!("{}Checking the sources{}", YELLOW, RESET);
                let mut file = File::open(src)?;
                let mut buffer = [0u8; 512];
                let bytes_read = file.read(&mut buffer)?;
                let is_archive = is_archive(bytes_read, &buffer)?;
                if is_archive == true {
                    println!("{} {}", src, collection);
                    env::set_current_dir(&building)?;
                    extract(&src.to_string())?;
                    env::set_current_dir(&collection)?;
                }
            }
        }
    }
    env::set_current_dir(&collection)?;
    let prepare = fs::read_to_string("Pkgfile").context("Failed to build pkgfile")?;
    let cmd = match (prepare.contains("prepare()"), prepare.contains("package()"), prepare.contains("build()")) {
        (true, true, true) => {
            format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && build && cd $SRC && package'")
        }
        (true, false, true) => {
            format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && build'")
        }
        (false, true, true) => {
            format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && build && cd $SRC && package'")
        }
        (false, false, true) => {
            format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && build'")
        }
        (true, true, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = match build_style.split_once("=").map(|(_, style)| style) {
                        Some(style) => style,
                        None => bail!("Invalid build= line"),
                    };

                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/{} && cd $SRC && package'", style)
                    } else {
                        bail!("No build style available for {}", build_style);
                    }
                }
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/build-default && cd $SRC && package'")
                    } else {
                        bail!("No default build style set in /etc/raw.d/build-default : aborting");
                    }
                }
            }
        }
        (true, false, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = match build_style.split_once("=").map(|(_, style)| style) {
                        Some(style) => style,
                        None => bail!("Invalid build= line"),
                    };

                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/{}'", style)
                    } else {
                        bail!("No build style available for {}", build_style);
                    }
                }
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && prepare && cd $SRC && source /etc/raw.d/build-default'")
                    } else {
                        bail!("No default build style set in /etc/raw.d/build-default : aborting");
                    }
                }
            }
        }
        (false, true, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = match build_style.split_once("=").map(|(_, style)| style) {
                        Some(style) => style,
                        None => bail!("Invalid build= line"),
                    };

                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/{} && cd $SRC && package'", style)
                    } else {
                        bail!("No build style available for {}", build_style);
                    }
                }
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/build-default && cd $SRC && package'")
                    } else {
                        bail!("No default build style set");
                    }
                }
            }
        }
        (false, false, false) => {
            match prepare.contains("build=") {
                true => {
                    let build_style = prepare.lines().find(|b| b.starts_with("build=")).unwrap();
                    let style = match build_style.split_once("=").map(|(_, style)| style) {
                        Some(style) => style,
                        None => bail!("Invalid build= line"),
                    };

                    if Path::new(&format!("/etc/raw.d/{}", style)).exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/{}'", style)
                    } else {
                        bail!("No build style available for {}", build_style);
                    }
                }
                false => {
                    if Path::new("/etc/raw.d/build-default").exists() {
                        format!("fakeroot bash -eo pipefail -c 'source Pkgfile && PKG=$(pwd)/pkg && SRC=$(pwd)/work && cd work && source /etc/raw.d/build-default'")
                    } else {
                        bail!("No default build style available");
                    }
                }
            }
        }
    };
    if !Path::new(&format!("{}/.local/share/raw/", env::var("HOME").unwrap())).exists() {
        fs::create_dir_all(format!("{}/.local/share/raw/", env::var("HOME").unwrap())).context("Failed to create log path")?;
    }
    let log_path = format!("{}/.local/share/raw/raw.log", env::var("HOME").unwrap());
    if Path::new(&log_path).exists() {
        fs::remove_file(&log_path).context("Failed to remove current log file")?;
        File::create(&log_path).context("Failed to create log file")?;
    } else {
        File::create(&log_path).context("Failed to  create log file")?;
    }
    let cmd = format!("{} 2>&1 | tee -a {}/.local/share/raw/raw.log", cmd, env::var("HOME").unwrap());
    let _output_build = match Command::new("bash")
    .args(["-eo", "pipefail", "-c", &cmd])
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
        fs::remove_dir_all("work").context("Failed to remove the work directory")?;
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
        anyhow::bail!("Build failed");
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
        anyhow::bail!("Build failed");
    }
    };
    let prepare = format!("{}/pkg", collection);

    if Path::new(&format!("{}.{}#raw.tar.gz", name, version)).exists() {
        println!("Removing the previous generated package");
        fs::remove_file(format!("{}.{}#raw.tar.gz", name, version))?;
    }
    println!("Generating footprint and looking for changes");
    if Path::new(&format!("{}.footprint", name)).exists() {
        let existing = fs::read_to_string(format!("{}.footprint", name)).context("Failed to read footprint")?;
        fs::remove_file(format!("{}.footprint", name)).context("Failed to remove footprint")?;
        let mut footprint = File::create(format!("{}.footprint", name)).context("Failed to open footprint")?;
        for entry in WalkDir::new(&prepare).follow_links(false) {
            let entry = entry?;
            let foot = entry.path().display().to_string();
            let pathpkg = foot.split_once(&prepare).map(|(_,pathpkg)| pathpkg).context("Not found")?.to_string();
            if pathpkg.is_empty() { continue; }
                let _list = pathpkg.split_once('/').map(|(_,list)| list).context("Failed to generate footprint")?.to_string();
                if entry.file_type().is_symlink() {
                    let list = pathpkg.split_once('/').map(|(_,list)| list).context("Failed to generate footprint")?.to_string();
                    let link = fs::read_link(entry.path())?;
                    writeln!(footprint, "{} -> {}", list, link.display())?;
                } else {
                    let list = pathpkg.split_once('/').map(|(_,list)| list).context("Failed to generate footprint")?.to_string();
                writeln!(footprint, "{}", list)?;
            }

        }
        let footprint = fs::read_to_string(format!("{}.footprint", name)).context("Footprint reading failed")?;
        if existing == footprint {
            println!("{}Footprint didn't change{}", YELLOW, RESET)
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
        let mut footprint = File::create(format!("{}.footprint", name)).context("Failed to create footprint")?;
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
    let footprint = fs::read_to_string(format!("{}.footprint", name)).context("Failed to read footprint")?;
    println!("{}This package contains : {} files{}", YELLOW, footprint.lines().count(), RESET);
    fs::copy("META", "pkg/META").context("Failed to copy META file to prepare for compression")?;
    fs::remove_file("META").unwrap();
    fs::copy(format!("{}.footprint", name), format!("pkg/{}.footprint", name)).context("Failed to prepare footprint file for compression")?;
    if Path::new("automatic").exists() {
        fs::copy("automatic", "pkg/automatic")?;
    }
    if Path::new(&format!("{}/{}.pre-install", collection, name)).exists() {
        fs::copy(format!("{}.pre-install", name), format!("pkg/{}.pre-install", name)).context("Failed to package pre installation file")?;
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
    let building = building.split_once("/work").map(|(building, _)| building).context("Failed to find pkg/")?;
    let building = format!("{}/pkg", building);
    println!("{}", building);
    let libs = scan_pkg_dir(Path::new(&building));
    println!("{}Runtime libraries found : {:?}{}", GREEN, libs, RESET);
    let mut pkgdeps = Vec::new();
    for i in libs {
        let pkgdep = query(&format!("{}", i));
        pkgdeps.push(pkgdep);
    } 
    let mut metar = String::new();
    if !pkgdeps.is_empty() {
        let pkgdeps: Vec<String> = pkgdeps.into_iter().filter_map(|l| l.ok()).flatten().collect();
        let pkgdeps = pkgdeps.join(" ");
        let meta = fs::read_to_string("pkg/META")?;
        for i in meta.lines() {
            if i.starts_with("R") {
                metar = i.split_once("R").map(|(_, metar)| metar).context("Rundepends line not found")?.to_string();
            }
        }
        let new_content: String = meta
            .lines()
            .filter(|line| !line.starts_with("R"))
            .collect::<Vec<&str>>()
            .join("\n");
        
        if !metar.is_empty() {
            fs::write("pkg/META", format!("{}\nR{} {}\n", new_content, metar, pkgdeps))?;
        } else {
            fs::write("pkg/META", format!("{}\nR{}\n", new_content, pkgdeps))?;
        }
    }
    println!("Generating package");
    for i in fs::read_dir(".")? {
        let i = i?;
        let i = i.file_name().to_string_lossy().to_string();
        if i.contains(".raw.") {
            fs::remove_file(i)?;
        }
    }
    let tar = File::create(format!("{}.{}#{}.raw.tar.gz", name, version, release))?;
    let enc = GzEncoder::new(tar, Compression::default());
    let mut a = tar::Builder::new(enc);
    a.follow_symlinks(false);
    a.append_dir_all("", "pkg/")?;
    let mut gz = a.into_inner().context("Tar failed")?;
    gz.try_finish().context("Gzip flush failed")?;
    fs::remove_dir_all("pkg")?;
    createsha(&format!("{}.{}#{}.raw.tar.gz", name, version, release))?;
    Ok(())
}



pub fn is_archive(bytes_read: usize, buffer: &[u8]) -> Result<(bool)> {
    let mut is_archive = false;
    for sig in SIGNATURES {
        let start = sig.offset as usize;
        let end = start + sig.magic.len();
        if bytes_read >= end && &buffer[start..end] == sig.magic {
            is_archive = true;
            break;
        }
    }
    return Ok(is_archive)
}