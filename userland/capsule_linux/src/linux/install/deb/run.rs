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

//! `install deb:<name>`: the package and everything it depends on, taking
//! the first alternative of each need the index has.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::fetch::fetch;
use super::index::{find, load};
use super::keyring::pinned;
use super::packages::Record;
use super::source::source;
use crate::linux::install::limit::max_packages;
use crate::linux::install::place::unpack;

pub fn install(name: &str, pin: &[u8; 32]) -> bool {
    let Some(src) = source() else {
        return say(b"[LINUX] this image was built without a Debian mirror\n");
    };
    let Some(ring) = pinned() else {
        return say(b"[LINUX] this image pins no Debian archive key\n");
    };
    let Some(index) = load(&src, &ring) else {
        return say(b"[LINUX] no verified Debian index\n");
    };
    let (max, mut done): (usize, Vec<String>) = (max_packages(), Vec::new());
    let mut wanted: Vec<Vec<String>> = vec![vec![String::from(name)]];
    while let Some(group) = wanted.pop() {
        let Some(rec): Option<&Record> = group.iter().find_map(|n| find(&index, n)) else {
            return say(b"[LINUX] nothing provides it\n");
        };
        if done.contains(&rec.name) {
            continue;
        }
        if done.len() == max {
            return say(
                alloc::format!("[LINUX] refused: closure passes {max} packages\n").as_bytes()
            );
        }
        let chosen = rec.name == name;
        let Some(files) = fetch(&src, rec, chosen.then_some(pin)) else {
            return false;
        };
        unpack(&files, chosen.then_some(name));
        done.push(rec.name.clone());
        wanted.extend(rec.depends.iter().cloned());
    }
    true
}

/// Log a refusal; every caller is returning failure.
fn say(line: &[u8]) -> bool {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    false
}
