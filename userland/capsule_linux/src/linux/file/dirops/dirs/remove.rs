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

/* rmdir, and a directory removed by unlinkat's AT_REMOVEDIR. */

use super::super::super::at::resolve_at;
use super::super::super::meta::look;
use super::super::super::resolve::key;
use super::super::super::{cache, modes, store_name, synth};
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub fn rmdir(guest: &Guest, path: u64) -> u64 {
    let at = match resolve_at(guest, super::super::super::flags::AT_FDCWD, path) {
        Ok(at) => at,
        Err(e) => return errno::fail(e),
    };
    remove_dir(&at)
}

/*
 * Not recursive: POSIX rmdir refuses a populated directory, and a recursive
 * delete behind that name is data loss. A file the family holds and has not
 * yet put in the store is in the directory all the same.
 */
pub(in super::super) fn remove_dir(at: &[u8]) -> u64 {
    match look(at) {
        None => return errno::fail(errno::ENOENT),
        Some((_, false)) => return errno::fail(errno::ENOTDIR),
        Some(_) if synth::owns(at) || key(at).writable().is_err() => {
            return errno::fail(errno::EROFS)
        }
        Some(_) if !cache::names_in(at).is_empty() => return errno::fail(errno::ENOTEMPTY),
        Some(_) => {}
    }
    match store_name::rmdir(&key(at)) {
        Ok(()) => {
            modes::forget(at);
            errno::ok(0)
        }
        Err(_) => errno::fail(errno::ENOTEMPTY),
    }
}
