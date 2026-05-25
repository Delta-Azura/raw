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
use compress_tools::*;
use std::path::Path;
use anyhow::{Result};
use anyhow::Context;


pub fn extract(tarball: &String) -> Result<()> {
    //let source = File::open(tarball).unwrap();
    let mut source = File::open(tarball)?;
    let dest = Path::new(".");
    uncompress_archive(&mut source, &dest, Ownership::Ignore).context("Uncompressing failed")?;
    Ok(())
}


pub async fn extract_parallel(tarball: &String) -> Result<()> {
    let mut source = File::open(tarball)?;
    let dest = Path::new(".");
    uncompress_archive(&mut source, &dest, Ownership::Ignore).context("Uncompressing failed")?;
    Ok(())
}