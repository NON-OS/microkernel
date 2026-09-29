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

/* Opening a model: read-only, sized from the volume, imported if pinned. */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::flags::{wants_read, O_CREAT};
use super::super::{desc, slot};
use super::name::{owns, volume_name, ROOT};
use super::size::size_of;

/* Open `path`, already followed, if it is under /models; None if it is not. */
pub fn open(guest: &mut Guest, path: &[u8], flags: u64) -> Option<u64> {
    if !owns(path) {
        return None;
    }
    let Some(name) = volume_name(path) else {
        return Some(errno::fail(if path == ROOT { errno::EACCES } else { errno::ENOENT }));
    };
    /* O_WRONLY, O_RDWR or O_CREAT: the volume takes files only by import. */
    if flags & 3 != 0 || flags & O_CREAT != 0 {
        return Some(errno::fail(errno::EROFS));
    }
    let size = match size_of(name) {
        Ok(size) => size,
        Err(e) => return Some(errno::fail(e)),
    };
    let mut fd = Fd::file(path.to_vec(), size, None, false);
    fd.handle = desc::fresh(false, wants_read(flags));
    Some(match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    })
}
