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


const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[0;32m";
const YELLOW: &str = "\x1b[33m";


pub fn help() {
    println!("{}WELCOME TO RAW ! Here are a little set of tips for you :{}", YELLOW, RESET);
    println!("raw install packagename {}#to install a package{}", GREEN, RESET);
    println!("raw remove packagename {}#to remove it{}", GREEN, RESET);
    println!("raw update packagename {}#to update the package individually{}", GREEN, RESET);
    println!("raw index {}#to update/create the index{}", GREEN, RESET);
    println!("raw build {}#allows you to build a package regardless the directory you are in, add -y to install it afterwards{}", GREEN, RESET);
    println!("raw list {}#to list every packages installed{}", GREEN, RESET);
    println!("raw info packagename {}#to display informations about a package{}", GREEN, RESET);
    println!("raw libs packagename {}#to list its libraries{}", GREEN, RESET);
    println!("raw libs packagename all {}#to list the libraries including the lib/security ones{}", GREEN, RESET);
    println!("raw rmcache {}#to remove the cached packages, use it carefully{}", RED, RESET);
    println!("raw get packagename {}#only in binary mode to install a package{}", YELLOW, RESET);
    println!("raw upgrade {}#to upgrade your system, only in binary mode{}", YELLOW, RESET);
    println!("raw query filename {}#to know who this file belongs to{}", GREEN, RESET);
    println!("raw package {}#to build the package being in the directory containing its pkgfile{}", GREEN, RESET);
    println!("raw orphans {}#to list every orphans remaining on the system and their number{}", GREEN, RESET);
    println!("raw community pkg {}#To download a pkgfile for an aur like repository{}", GREEN, RESET);
    println!("raw template pkg{}#To create a directory containing a template of pkgfile{}", GREEN, RESET);
    println!("raw bootstrap pkg /path{}#To install a package in a specific directory{}", GREEN, RESET);
    println!("raw files pkg{}#To list the footprint of a selected package{}", GREEN, RESET);
    println!("raw search pkg{}#To search a package in the database{}", GREEN, RESET);
    println!("raw changelog {}#To list every upgradables packages{}", GREEN, RESET);

}