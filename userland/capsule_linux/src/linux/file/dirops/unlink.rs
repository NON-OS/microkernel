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

/* unlinkat: a file, a link, or with AT_REMOVEDIR a directory. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::at::resolve_at;
use super::super::meta::look;
use super::super::resolve::key;
use super::super::{cache, modes, store_name, synth};
use super::dirs::remove_dir;

pub fn unlinkat(guest: &Guest, dirfd: u64, path: u64, flags: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    /*
     * AT_REMOVEDIR turns unlinkat into rmdir, which is how a libc implements
     * rmdir on top of one syscall.
     */
    const AT_REMOVEDIR: u64 = 0x200;
    if flags & AT_REMOVEDIR != 0 {
        return remove_dir(&at);
    }
    if guest.links.target(&at).is_some() {
        return match key(&at).writable() {
            Ok(()) if guest.links.remove(&at) => errno::ok(0),
            _ => errno::fail(errno::EROFS),
        };
    }
    let held = cache::held(&at);
    match look(&at) {
        None => return errno::fail(errno::ENOENT),
        Some((_, true)) => return errno::fail(errno::EISDIR),
        Some(_) if synth::owns(&at) || key(&at).writable().is_err() => {
            return errno::fail(errno::EROFS)
        }
        Some(_) => {}
    }
    cache::forget(&at);
    modes::forget(&at);
    super::super::times::forget(&at);
    super::super::xattr_table::forget(&at);
    /* A file only the family held was never in the store. */
    match store_name::unlink(&key(&at)) {
        Ok(()) => errno::ok(0),
        Err(_) if held => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}
