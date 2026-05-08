use std::path::Path;
use std::fs;
use std::process::Command;
use std::env;
use recursive_copy::{copy_recursive, CopyOptions};
use crate::conflict::conflict;
use tar::Archive;
use flate2::read::GzDecoder;
use anyhow::{Result};
use anyhow::Context;
use std::fs::File;
use walkdir::WalkDir;
use std::io;
use crate::file_type::file_type;



pub fn bootstrap(rawpkg: &String, bootstrap_path: &str) -> Result<()> {
    println!("\x1b[31;1m[WARN] Please use this function only to install base packages, it will not run any post installation nor ldconfig !\x1b[0m");
    //let pkg_name = rawpkg.split_once(".raw").map(|(name, _)| name).unwrap_or(rawpkg);
    File::create("/var/cache/tmp.raw").context("Not running as root, aborting")?;
    fs::remove_file("/var/cache/tmp.raw").unwrap();
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap();
    fs::create_dir(format!("{}/var/lib/pkg/DB/", bootstrap_path)).unwrap();
    fs::create_dir(format!("{}/var/lib/pkg/DB/{}", bootstrap_path, pkg)).unwrap();
    println!("Copying {} to /var/lib/pkg/DB/{}/{} in bootstrap directory", rawpkg, pkg, rawpkg);
    fs::copy(rawpkg, format!("{}/var/lib/pkg/DB/{}/{}", bootstrap_path, pkg, rawpkg)).unwrap();
    env::set_current_dir(format!("{}/var/lib/pkg/DB/{}", bootstrap_path, pkg)).unwrap();
    if rawpkg.ends_with(".tar.gz") || rawpkg.ends_with(".tgz") {
        let file = fs::File::open(rawpkg).unwrap();
        let mut archive = Archive::new(GzDecoder::new(file));
        archive.unpack(".").unwrap();
    } else {
        println!("No package in the format required : ABORTING");
        std::process::exit(1);
    }
    let opts = CopyOptions {
        overwrite: true,
        follow_symlinks: false,
        restrict_symlinks: false,
        content_only: false,
        ..Default::default()
    };
    copy_recursive(Path::new("."), Path::new(bootstrap_path), &opts).unwrap();
    fs::remove_dir_all(format!("{}/var/lib/pkg/DB/{}", bootstrap_path, pkg)).unwrap();
    fs::create_dir(format!("{}/var/lib/pkg/DB/{}", bootstrap_path, pkg)).unwrap();
    fs::copy(format!("{}/META", bootstrap_path), format!("{}/var/lib/pkg/DB/{}/META", bootstrap_path, pkg)).unwrap();
    fs::copy(format!("{}/{}.footprint", bootstrap_path, pkg), format!("{}/var/lib/pkg/DB/{}/files", bootstrap_path, pkg)).unwrap();
    fs::remove_file(format!("{}/META", bootstrap_path)).unwrap();
    fs::remove_file(format!("{}/{}.footprint", bootstrap_path, pkg)).unwrap();
    fs::remove_file(format!("{}/{}", bootstrap_path, rawpkg)).unwrap();
    //let content = line.lines();
    Ok(())
}