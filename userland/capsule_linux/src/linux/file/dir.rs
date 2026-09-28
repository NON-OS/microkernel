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

/*
 * Directory open. The listing is snapshotted here, which is all POSIX
 * promises a directory stream.
 */

use alloc::string::String;
use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::dir_children::children;
use super::{cache, desc, resolve, slot, store, synth};

pub fn open(guest: &mut Guest, path: Vec<u8>) -> u64 {
    let at = resolve::key(&path);
    let Ok(keys) = store::list(&at) else {
        return errno::fail(errno::EACCES);
    };
    /*
     * Cut against the store key, not against the path the guest named.
     * Every Linux directory lists itself and its parent first.
     */
    let mut names = alloc::vec![String::from("."), String::from("..")];
    names.extend(children(at.as_bytes(), keys));
    let made = if path == b"/" {
        synth::ROOTS.iter().map(|r| String::from(*r)).collect()
    } else {
        Vec::new()
    };
    for name in guest.links.names_in(&path).into_iter().chain(cache::names_in(&path)).chain(made) {
        if !names.contains(&name) {
            names.push(name);
        }
    }
    let mut fd = Fd::dir(path, names);
    fd.handle = desc::fresh(false, false);
    match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
