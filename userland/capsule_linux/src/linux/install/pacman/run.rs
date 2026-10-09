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

//! `install pacman:<name>`: the package and everything it depends on.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::db::load;
use super::fetch::fetch;
use super::keyring::pinned;
use super::source::source;
use crate::linux::install::limit::max_packages;
use crate::linux::install::place::unpack;
use crate::linux::install::Why;

pub fn install(name: &str, pin: &[u8; 32]) -> Result<(), Why> {
    let Some(src) = source() else {
        return say(b"[LINUX] this image was built without a pacman mirror\n", Why::NoMirror);
    };
    let Some(ring) = pinned() else {
        return say(b"[LINUX] this image pins no pacman keyring\n", Why::NoKeyring);
    };
    super::super::http::reachable(src.at())?;
    let Some(db) = load(&src, &ring) else {
        return say(b"[LINUX] no verified pacman database\n", Why::Index);
    };
    let max = max_packages();
    let mut wanted: Vec<String> = vec![String::from(name)];
    let mut done: Vec<String> = Vec::new();
    while let Some(next) = wanted.pop() {
        let Some((rec, repo)) = db.find(&next) else {
            return say(b"[LINUX] nothing provides it\n", Why::NotProvided);
        };
        if done.contains(&rec.name) {
            continue;
        }
        if done.len() == max {
            let line = alloc::format!("[LINUX] refused: closure passes {max} packages\n");
            return say(line.as_bytes(), Why::TooLarge);
        }
        let chosen = rec.name == name;
        let Some(files) = fetch(&src, &ring, repo, rec, chosen.then_some(pin)) else {
            return Err(Why::Package);
        };
        unpack(&files, &rec.name, chosen)?;
        done.push(rec.name.clone());
        wanted.extend(rec.depends.iter().cloned());
    }
    Ok(())
}

/// Log a refusal and name it.
fn say(line: &[u8], why: Why) -> Result<(), Why> {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    Err(why)
}
