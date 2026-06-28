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


pub mod install;
pub mod conflict;
pub mod info;
pub mod query;
pub mod remove;
pub mod update;
pub mod package;
pub mod download;
pub mod extract;
pub mod file_type;
pub mod files;
pub mod list;
pub mod librs;
pub mod getconf;
pub mod index;
pub mod get;
pub mod depends;
pub mod upgrade;
pub mod bootstrap;
pub mod search;
pub mod remove_cache;
pub mod help;
pub mod orphans;
pub mod template;
pub mod num_cpus;
pub mod getlibs;
pub mod sig;
pub mod localpkg;
pub mod diff;
pub mod changelog;

#[allow(unused_imports)]
pub use crate::changelog::changelog;
pub use crate::getlibs::get_needed_libs;
pub use crate::diff::diff;
pub use crate::sig::createsha;
pub use crate::localpkg::localpkg;
pub use crate::template::template;
pub use crate::orphans::orphans;
pub use crate::help::help;
pub use crate::bootstrap::bootstrap;
pub use crate::get::get;
pub use crate::index::index;
pub use crate::getconf::getconf;
pub use crate::librs::libs;
pub use crate::list::list;
pub use crate::files::files;
pub use crate::install::install;
pub use crate::info::info;
pub use crate::query::query;
pub use crate::remove::remove;
pub use crate::update::update;
pub use crate::package::package;
pub use crate::upgrade::upgrade;
pub use crate::search::search;
pub use crate::remove_cache::remove_cache;