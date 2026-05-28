use std::env;
use anyhow::{Result, Context};
use crate::package;
use std::fs;
use crate::getconf;

pub fn build(pkg: &str) -> Result<()> {
    getconf().unwrap();
    //env::set_current_dir(&pkg)?;
    let path = fs::read_to_string("index.raw").context("index.raw doesn't exist, please run raw index")?;
    let path = path.lines().find(|l| l.contains(&format!("{}/", pkg))).context("This package doesn't exists on the index")?.split_once("/Pkgfile").map(|(path, _)| path).unwrap().to_string();
    println!("{}", path);
    env::set_current_dir(path)?;
    package(None)?;
    Ok(())
}