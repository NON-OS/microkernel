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

//! Opening a name in the person's folders.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_user_files::Kind as Node;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::flags::{wants_read, writes, O_APPEND};
use super::super::{desc, slot};
use super::{held, link, list, place};

/// Opens `full`, already followed, when it is in the person's folders and
/// this guest is served them; None when it is not the broker's.
pub fn open(guest: &mut Guest, full: &[u8], flags: u64, mode: u64) -> Option<u64> {
    let path = place::served(guest, full)?;
    let mode = (mode & 0o7777) as u32;
    let (handle, attr) = match link::with(|c| c.open(path, super::flags::of(flags), mode)) {
        Ok(got) => got,
        Err(e) => return Some(errno::fail(e as i64)),
    };
    if attr.kind == Node::Dir {
        // A directory is listed once, at open; the broker keeps no handle for it.
        let _ = link::with(|c| c.close(handle));
        return Some(match list::names(path) {
            Ok(names) => install(guest, Fd::dir(full.to_vec(), names), None),
            Err(e) => errno::fail(e as i64),
        });
    }
    let mut fd = Fd::file(full.to_vec(), attr.size, None, writes(flags));
    fd.handle = desc::fresh(flags & O_APPEND != 0, wants_read(flags));
    Some(install(guest, fd, Some(handle)))
}

fn install(guest: &mut Guest, mut fd: Fd, handle: Option<u64>) -> u64 {
    if handle.is_none() {
        fd.handle = desc::fresh(false, false);
    }
    let d = desc::of(&fd);
    match slot::install(guest, fd) {
        Some(n) => {
            if let (Some(d), Some(h)) = (d, handle) {
                held::hold(d, h);
            }
            errno::ok(n)
        }
        None => {
            if let Some(h) = handle {
                let _ = link::with(|c| c.close(h));
            }
            errno::fail(errno::EMFILE)
        }
    }
}

/// A listing's names, for a directory descriptor.
pub(super) fn strings(names: Vec<Vec<u8>>) -> Vec<String> {
    names.into_iter().map(|n| String::from_utf8_lossy(&n).into_owned()).collect()
}
