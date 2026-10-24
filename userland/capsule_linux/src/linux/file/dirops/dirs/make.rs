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

/* mkdir and mkdirat, in the family's private directories. */

use super::super::super::at::resolve_at;
use super::super::super::meta::look;
use super::super::super::resolve::key;
use super::super::super::space::{room_for, Kept};
use super::super::super::{modes, store_name, synth};
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub fn mkdirat(guest: &Guest, dirfd: u64, path: u64, mode: u64) -> u64 {
    let at = match resolve_at(guest, dirfd, path) {
        Ok(at) => at,
        Err(e) => return errno::fail(e),
    };
    if look(&at).is_some() || guest.links.target(&at).is_some() {
        return errno::fail(errno::EEXIST);
    }
    if synth::refused_write(&at) || key(&at).writable().is_err() {
        return errno::fail(errno::EROFS);
    }
    /* A directory is a name the private directories' quota counts. */
    if let Err(e) = room_for(Kept { bytes: 0, names: 1 }) {
        return errno::fail(e);
    }
    match store_name::mkdir(&key(&at)) {
        Ok(()) => {
            modes::set(&at, mode as u32 & !u32::from(guest.umask));
            errno::ok(0)
        }
        Err(_) => errno::fail(errno::ENOENT),
    }
}
