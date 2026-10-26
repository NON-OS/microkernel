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

/* linkat: a hard link, which here is a copy of the file. */

use super::super::super::at::resolve_at;
use super::super::super::meta::look;
use super::super::super::resolve::key;
use super::super::super::space::{room_for, Kept};
use super::super::super::{store_read, store_stat, store_write};
use super::super::across::same_mount;
use super::super::free::free_and_writable;
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/* Largest file a hard link copies; the same bound an exec image has. */
const MAX_LINKED: u32 = 64 << 20;

pub fn linkat(guest: &Guest, olddir: u64, old: u64, newdir: u64, new: u64) -> u64 {
    let from = match resolve_at(guest, olddir, old) {
        Ok(from) => from,
        Err(e) => return errno::fail(e),
    };
    let at = match resolve_at(guest, newdir, new) {
        Ok(at) => at,
        Err(e) => return errno::fail(e),
    };
    let from = super::super::super::walk::follow(guest, from, true);
    if let Err(e) = free_and_writable(guest, &at) {
        return errno::fail(e);
    }
    if look(&from).is_none() {
        return errno::fail(errno::ENOENT);
    }
    if let Err(e) = same_mount(&from, &at) {
        return errno::fail(e);
    }
    /* The store copies what it has: the family's copy goes in first. */
    if let Err(e) = super::super::super::cache::flush(&from, true) {
        return errno::fail(e);
    }
    /* The copy is a new name and its bytes, both within the quota. */
    let size = match store_stat(&key(&from)) {
        Ok((size, false)) => size,
        /* Linux never links a directory. */
        Ok((_, true)) => return errno::fail(errno::EPERM),
        Err(_) => return errno::fail(errno::ENOENT),
    };
    if let Err(e) = room_for(&at, Kept { bytes: size, names: 1 }) {
        return errno::fail(e);
    }
    let Ok(bytes) = store_read(&key(&from), MAX_LINKED) else {
        return errno::fail(errno::ENOENT);
    };
    match store_write(&key(&at), &bytes) {
        Ok(()) => errno::ok(0),
        Err(e) => errno::fail(super::super::super::store_err::errno_of(e)),
    }
}
