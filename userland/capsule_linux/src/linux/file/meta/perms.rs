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

/* Mode bits, and whether a path can be reached. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::at::resolve_at;
use super::super::flags::AT_FDCWD;
use super::super::{cache, modes, resolve, synth, walk};
use super::stat::look;

pub fn fchmodat(guest: &Guest, dirfd: u64, path: u64, mode: u64) -> u64 {
    let at = match resolve_at(guest, dirfd, path) {
        Ok(at) => at,
        Err(e) => return errno::fail(e),
    };
    change(&walk::follow(guest, at, true), mode)
}

pub fn fchmod(guest: &Guest, fd: u64, mode: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    if entry.kind != Kind::File && entry.kind != Kind::Dir {
        return errno::fail(errno::EINVAL);
    }
    change(&entry.path.clone(), mode)
}

/*
 * The store keeps no modes, so the family does (held/modes.rs). The shared
 * tree and /dev, /proc and /sys are mounted read-only.
 */
fn change(full: &[u8], mode: u64) -> u64 {
    if look(full).is_none() {
        return errno::fail(errno::ENOENT);
    }
    if synth::owns(full) || (!cache::held(full) && resolve::key(full).writable().is_err()) {
        return errno::fail(errno::EROFS);
    }
    modes::set(full, mode as u32);
    errno::ok(0)
}

pub fn chmod(guest: &Guest, path: u64, mode: u64) -> u64 {
    fchmodat(guest, AT_FDCWD, path, mode)
}
