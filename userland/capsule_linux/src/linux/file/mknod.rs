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

/* `mknodat`: a regular file is an empty file; no device node or fifo is made. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::at::resolve_at;
use super::meta::stat;
use super::owner::refused;
use super::resolve::key;

const S_IFMT: u64 = 0o170000;
const S_IFREG: u64 = 0o100000;

/* A regular file is an empty file; devices and fifos are not made here. */
pub fn mknodat(guest: &Guest, dirfd: u64, path: u64, mode: u64) -> u64 {
    if mode & S_IFMT != S_IFREG && mode & S_IFMT != 0 {
        return refused(b"[LINUX] refused mknod: no device nodes or fifos\n");
    }
    let at = match resolve_at(guest, dirfd, path) {
        Ok(at) => at,
        Err(e) => return errno::fail(e),
    };
    if stat::look(&at).is_some() {
        return errno::fail(errno::EEXIST);
    }
    if super::synth::owns(&at) || key(&at).writable().is_err() {
        return errno::fail(errno::EROFS);
    }
    /* A new name, which the private directories' quota counts. */
    if let Err(e) = super::space::room_for(super::space::Kept { bytes: 0, names: 1 }) {
        return errno::fail(e);
    }
    match super::store_write(&key(&at), &[]) {
        Ok(()) => {
            super::modes::set(&at, mode as u32 & 0o7777 & !u32::from(guest.umask));
            errno::ok(0)
        }
        Err(e) => errno::fail(super::store_err::errno_of(e)),
    }
}
