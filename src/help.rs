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
    println!("raw build {}#allows you to build a package regardless the directory you are in{}", GREEN, RESET);
    println!("raw list {}#to list every packages installed{}", GREEN, RESET);
    println!("raw info packagename {}#to display informations about a package{}", GREEN, RESET);
    println!("raw libs packagename {}#to list its libraries{}", GREEN, RESET);
    println!("raw libs packagename all {}#to list the libraries including the lib/security ones{}", GREEN, RESET);
    println!("raw rmcache {}#to remove the cached packages, use it carefully{}", RED, RESET);
    println!("raw get packagename {}#only in binary mode to install a package{}", YELLOW, RESET);
    println!("raw upgrade {}#to upgrade your system, only in binary mode{}", YELLOW, RESET);
    println!("raw query filename {}#to know who this file belongs to{}", GREEN, RESET);
    println!("raw package {}#to build the package being in the directory containing it's pkgfile{}", GREEN, RESET);
}