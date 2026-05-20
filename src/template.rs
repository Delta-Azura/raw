use std::fs::OpenOptions;
use anyhow::{Result, Context};
use std::io::Write;
use std::fs::File;
use std::env;
use std::fs;
use crate::getconf;
use std::path::Path;


pub fn template(pkg: &str) -> Result<()> {
    match File::create("/var/cache/raw.tmp") {
        Ok(_) => {
            println!("You are building as root !");
            fs::remove_file("/var/cache/raw.tmp")?;
            std::process::exit(1)
        }
        Err(e) => {}
    }
    let pwd = env::current_dir()?;
    let (mode, root, trash) = getconf().unwrap();
    if mode != "source" {
        println!("Creating in binary mode");
    } else {
        if pwd.to_string_lossy().to_string() != root {
            println!("You are not creating the template in your directory set in your raw.conf, it will not work with raw index and raw build");
            env::set_current_dir(pwd).unwrap();
        }
    }
    if Path::new(pkg).exists() {
        fs::remove_dir_all(pkg)?;
    }
    fs::create_dir(pkg).context("Needs to be root, cannot initiate as packages needs to be built as non-root")?;
    env::set_current_dir(pkg).context("Invalid directory")?;
    File::create("Pkgfile").context("Failed to create PKgfile")?;
    let mut pkgfile = OpenOptions::new().append(true).write(true).open("Pkgfile")?;
    let content_pkgfile = "description=\nname=\nrelease=\nversion=\nmakedepends=\nrundepends=\nsource=\nbuild() {\n\n}\n";
    writeln!(pkgfile, "{}", content_pkgfile).context("Failed to write pkgfile")?;
    Ok(())
}



