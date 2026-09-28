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
//! `select`, answered from the same readiness the poll path reports.

use alloc::vec;
use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::{ready, POLLERR, POLLHUP};

/// What makes a descriptor count as ready in each set, as Linux's select
/// counts it: end of file is readable, and an error is both.
const POLLIN: u16 = 0x001 | POLLHUP | POLLERR;
const POLLOUT: u16 = 0x004 | POLLERR;

/// Linux caps a descriptor set at 1024 bits and so does every libc that
/// builds one, so a larger nfds is a caller error rather than a bigger set.
const FD_SETSIZE: u64 = 1024;

/// The bytes of a set that nfds covers, in whole longs, as Linux copies them.
fn set_bytes(nfds: u64) -> usize {
    (nfds.div_ceil(64) * 8) as usize
}

/// Count what is ready among `[readfds, writefds, exceptfds]`, narrowing
/// the sets to it. They are written back only when something is ready,
/// since a select that waits is tried again with the same sets; `clear`
/// empties them when its time runs out. Nothing here has an exceptional
/// condition, so that set always comes back empty.
pub fn select(guest: &mut Guest, nfds: u64, sets: [u64; 3]) -> u64 {
    if nfds > FD_SETSIZE {
        return errno::fail(errno::EINVAL);
    }
    let bytes = set_bytes(nfds);
    let mut narrowed: Vec<(u64, Vec<u8>)> = Vec::new();
    let mut hits = 0u64;
    for (at, want) in [(sets[0], POLLIN), (sets[1], POLLOUT)] {
        if at == 0 {
            continue;
        }
        let Some(mut set) = guest.read(at, bytes) else {
            return errno::fail(errno::EFAULT);
        };
        hits += narrow(guest, &mut set, nfds, want);
        narrowed.push((at, set));
    }
    if hits == 0 {
        return errno::ok(0);
    }
    if sets[2] != 0 {
        narrowed.push((sets[2], vec![0; bytes]));
    }
    for (at, set) in narrowed {
        if guest.write(at, &set) < 0 {
            return errno::fail(errno::EFAULT);
        }
    }
    errno::ok(hits)
}

/// The sets as a select whose time ran out leaves them: empty.
pub fn clear(guest: &mut Guest, nfds: u64, sets: [u64; 3]) {
    let empty = vec![0u8; set_bytes(nfds.min(FD_SETSIZE))];
    for at in sets.into_iter().filter(|&at| at != 0) {
        let _ = guest.write(at, &empty);
    }
}

/// Clear every bit whose descriptor is not ready for `want`, and report
/// how many were left set.
fn narrow(guest: &Guest, set: &mut [u8], nfds: u64, want: u16) -> u64 {
    let mut kept = 0;
    for fd in 0..nfds {
        let (byte, bit) = ((fd / 8) as usize, (fd % 8) as u32);
        if set[byte] & (1 << bit) == 0 {
            continue;
        }
        if ready(guest, fd) & want != 0 {
            kept += 1;
        } else {
            set[byte] &= !(1 << bit);
        }
    }
    kept
}
