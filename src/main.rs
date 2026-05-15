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

mod install;
mod conflict;
mod info;
mod query;
mod remove;
mod update;
mod package;
mod download;
mod extract;
mod file_type;
mod files;
mod list;
mod libs;
mod getconf;
mod index;
mod build;
mod get;
mod depends;
mod upgrade;
mod bootstrap;
mod search;
mod remove_cache;
mod help;
mod orphans;
use crate::orphans::orphans;
use crate::help::help;
use crate::bootstrap::bootstrap;
use crate::get::get;
use crate::build::build;
use crate::index::index;
use crate::getconf::getconf;
use crate::libs::libs;
use crate::list::list;
use crate::files::files;
use crate::install::install;
use crate::info::info;
use crate::query::query;
use crate::remove::remove;
use crate::update::update;
use crate::package::package;
use anyhow::{Result};
use crate::upgrade::upgrade;
use crate::search::search;
use crate::remove_cache::remove_cache;



fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        help();
        std::process::exit(0)
    }
    if args[1] == "help" {
        help();
        std::process::exit(0)
    
    }
    if args[1] == "--help" {
        help();
        std::process::exit(0)
    }
    if args[1] == "-help" {
        help();
        std::process::exit(0)
    }
    if args[1] == "h" {
        help();
        std::process::exit(0)
    }
    if args[1] == "package" {
        package()?;
        return Ok(());
    } 

    if args[1] == "install" {
        let argument = format!("{}", args[2]);
        install(&argument)?;
        return Ok(());
    }

    if args[1] == "info" {
        let argument = format!("{}", args[2]);
        info(&argument)?;
        return Ok(())
    }
    if args[1] == "remove" {
        let argument = format!("{}", args[2]);
        remove(&argument)?;
        return Ok(())
    }
    if args[1] == "query" {
        let argument = format!("{}", args[2]);
        query(&argument);
        return Ok(())
    }
    if args[1] == "update" {
        let argument = format!("{}", args[2]);
        update(&argument);
        return Ok(())
    }
    if args[1] == "files" {
        let argument = format!("{}", args[2]);
        files(&argument)?;
        return Ok(())
    } 
    if args[1] == "list" {
        list();
        return Ok(())
    } 
    if args[1] == "libs" {
        libs(&args[2], args.get(3).map(|s| s.as_str()))?;
        return Ok(())
    }
    if args[1] == "getconf" {
        println!("{:?}", getconf());
        return Ok(())
    }
    if args[1] == "index" {
        index()?;
        return Ok(())
    }
    if args[1] == "build" {
        build(&args[2], args.get(3).map(|s| s.as_str()))?;
        return Ok(())
    }
    if args[1] == "get" {
        get(&args[2])?;
        return Ok(())
    }
    if args[1] == "upgrade" {
        upgrade()?;
        return Ok(())
    }
    if args[1] == "bootstrap" {
        bootstrap(&args[2], &args[3])?;
        return Ok(())
    }
    if args[1] == "search" {
        search(&args[2])?;
        return Ok(())
    }
    if args[1] == "rmcache" {
        remove_cache()?;
        return Ok(())
    }
    if args[1] == "orphans" {
        orphans();
        return Ok(())
    }
    return Ok(());
}

