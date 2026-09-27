// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! `install <name>`: fetch a package and everything under it.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_debug;

use super::fetch::fetch;
use super::index_load::load_index;
use super::place::unpack;

pub(super) const RELEASE: &str = "v3.20";
pub(super) const ARCH: &str = "x86_64";
pub(super) const BRANCHES: [&str; 2] = ["main", "community"];

pub fn install(name: &str, pin: &[u8; 32]) -> bool {
    let Some(index) = load_index() else {
        say(b"[LINUX] no package index\n");
        return false;
    };
    let max = super::limit::max_packages();
    let mut wanted: Vec<String> = vec![String::from(name)];
    let mut done: Vec<String> = Vec::new();
    while let Some(next) = wanted.pop() {
        let found = match next.strip_prefix("so:") {
            Some(lib) => index.by_lib(lib),
            None => index.by_name(&next),
        };
        let Some(pkg) = found else {
            say(b"[LINUX] nothing provides it\n");
            return false;
        };
        if done.contains(&pkg.name) {
            continue;
        }
        if done.len() == max {
            let line = alloc::format!("[LINUX] refused: closure passes {max} packages\n");
            say(line.as_bytes());
            return false;
        }
        let Some(files) = fetch(pkg, (pkg.name == name).then_some(pin)) else {
            return false;
        };
        say(b"[LINUX] provenance Verified: index signature and checksums match\n");
        unpack(&files, (pkg.name == name).then_some(name));
        done.push(pkg.name.clone());
        wanted.extend(pkg.depends.iter().cloned());
    }
    true
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
