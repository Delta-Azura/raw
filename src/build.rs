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


use anyhow::{Result, Context};
use crate::package::package;
//use std::fs::File;
use std::env;
use std::fs;
use crate::getconf::getconf;
use std::path::Path;
use crate::install::install;
use crate::update::update;
use question::{Answer, Question};
use std::process::Command;

//use users::switch::switch_user_group;

const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[0;32m";

pub fn build(to_build: &str, option: Option<&str>) -> Result<()> {
    let (mode, trash, url) = getconf().unwrap();
    if mode != "source" {
        println!("{} Raw is used in binary mode, cannot build {}", RED, RESET);
        std::process::exit(1);
    }
    if let Ok(mode) = getconf() {
        let index = fs::read_to_string("index.raw").unwrap();
        let test = format!("/{}/", to_build);
        let found = index.lines().find(|line| line.contains(&test));
        if let Some(building) = found {
            let chrp = found.unwrap().split_once("Pkgfile").map(|(chrp, _)| chrp).unwrap();
            println!("{}", chrp);
            env::set_current_dir(&chrp).unwrap();
            let potential_package: Vec<String> = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).filter_map(|e| e.file_name().into_string().ok()).collect();
            if option != Some("-y") {
                for i in &potential_package {
                    if i.contains(".raw.") {
                        let question = Question::new(&format!("{} already exists, do you want to install it ?", i))
                            .yes_no()
                            .until_acceptable()
                            .default(Answer::YES)
                            .show_defaults()
                            .clarification("Please enter either 'y' or 'n' \n")
                            .ask();
                        if question == Some(Answer::YES) {
                            if Path::new(&format!("/var/lib/pkg/DB/{}", &to_build)).exists() {
                                Command::new("sudo")
                                .args(["raw", "update", &i])
                                .status();
                        //install(&i)?;
                                std::process::exit(0)
                            } else {
                                Command::new("sudo")
                                .args(["raw", "install", &i])
                                .status();
                                std::process::exit(0)
                            }
                        } else {
                            package(None).context("Build style or any other thing in the pkgfile might be incorrect. Try running package to know what's going on")?;
                        }
                    }
                }
            }
            if potential_package.contains(&format!(".raw.")) {
                if option == Some("-y") {
                    if Path::new(&format!("/var/lib/pkg/DB/{}", to_build)).exists() {
                        let content = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).map(|e| e.file_name().to_str().unwrap().to_owned()).find(|name| name.contains("raw"));
                        if Path::new("/usr/bin/sudo").exists() {
                            Command::new("sudo").args(["raw", "update", &content.unwrap()]).status().unwrap();
                        } else {
                            println!("{} sudo isn't installed, please go to the build directory to install {} {}", RED, to_build, RESET);
                        }
                    //drop(guard);
                    } else {
                        let content = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).map(|e| e.file_name().to_str().unwrap().to_owned()).find(|name| name.contains("raw"));
                        if Path::new("/usr/bin/sudo").exists() {
                            Command::new("sudo").args(["raw", "install", &content.unwrap()]).status().unwrap();
                        } else {
                            println!("{} sudo isn't installed, please go to the build directory to install {} {}", RED, to_build, RESET);
                        }
                    }

                    package(None).context("Build style or any other thing in the pkgfile might be incorrect. Try running package to know what's going on")?;
                    println!("{}Build succeded{}", GREEN, RESET);
                    if Path::new(&format!("/var/lib/pkg/DB/{}", to_build)).exists() {
                        let content = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).map(|e| e.file_name().to_str().unwrap().to_owned()).find(|name| name.contains("raw"));
                        if Path::new("/usr/bin/sudo").exists() {
                            Command::new("sudo").args(["raw", "update", &content.unwrap()]).status().unwrap();
                        } else {
                            println!("{} sudo isn't installed, please go to the build directory to install {} {}", RED, to_build, RESET);
                        }
                    //drop(guard);
                    } else {
                        let content = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).map(|e| e.file_name().to_str().unwrap().to_owned()).find(|name| name.contains("raw"));
                        if Path::new("/usr/bin/sudo").exists() {
                            Command::new("sudo").args(["raw", "install", &content.unwrap()]).status().unwrap();
                        } else {
                            println!("{} sudo isn't installed, please go to the build directory to install {} {}", RED, to_build, RESET);
                        }
                    }
                }
                //std::process::exit(0)
            }
        } else {
            println!("{} {} not found, try running raw index to update the repo database {}", RED, to_build, RESET);
            std::process::exit(1)
        }
    }
    Ok(())
}