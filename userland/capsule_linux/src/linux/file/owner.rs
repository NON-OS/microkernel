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

//! Owners and times: what the store does not record.
//!
//! The store keeps bytes under names and nothing else, so every file reports
//! uid 0, gid 0 and time zero, and the guest runs as uid 0. A change that
//! would leave that true is answered; one that would need the store to keep
//! something it cannot is refused by name, never reported done.

use nonos_libc::mk_debug;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::at::resolve_at;
use super::meta::stat;

const KEEP: u32 = u32::MAX;
const UTIME_OMIT: u64 = (1 << 30) - 2;

/// `fchownat`; `chown`, `lchown` and `fchown` are this at other bases.
pub fn fchownat(guest: &Guest, dirfd: u64, path: u64, uid: u64, gid: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    if stat::look(&guest.links.follow(at, true)).is_none() {
        return errno::fail(errno::ENOENT);
    }
    fchown_ids(uid, gid)
}

/// `fchown` on an open descriptor: only the owner every file already has.
pub fn fchown_ids(uid: u64, gid: u64) -> u64 {
    match [uid as u32, gid as u32].iter().all(|id| *id == 0 || *id == KEEP) {
        true => errno::ok(0),
        false => refused(b"[LINUX] refused chown: owners are not recorded\n"),
    }
}

/// Times are not kept, so only a call that changes neither is answered.
pub fn utimensat(guest: &Guest, times: u64) -> u64 {
    let omitted =
        |at: u64| guest.read(at + 8, 8).map(|n| u64::from_le_bytes(n.try_into().unwrap_or([0; 8])));
    if times != 0 && omitted(times) == Some(UTIME_OMIT) && omitted(times + 16) == Some(UTIME_OMIT) {
        return errno::ok(0);
    }
    refused(b"[LINUX] refused utimensat: times are not recorded\n")
}

pub(super) fn refused(line: &[u8]) -> u64 {
    let _ = mk_debug(line.as_ptr(), line.len());
    errno::fail(errno::EPERM)
}
