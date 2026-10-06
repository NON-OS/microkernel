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
//! `select`, answered from the same readiness the poll path reports. The
//! count and the sets are checked in `ready_sets`, which the host proofs hold.

use alloc::vec;
use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::file::MAX_FDS;
use crate::linux::guest::Guest;

use super::ready_sets::{first_closed, narrow, select_count, set_bytes};
use super::{ready, POLLERR, POLLHUP};

/// What makes a descriptor count as ready in each set, as Linux's select
/// counts it: end of file is readable, and an error is both.
const POLLIN: u16 = 0x001 | POLLHUP | POLLERR;
const POLLOUT: u16 = 0x004 | POLLERR;

/// Count what is ready among `[readfds, writefds, exceptfds]`, narrowing
/// the sets to it. In Linux's order: a negative count is EINVAL, a set that
/// cannot be read EFAULT, and a set naming a descriptor that is not open
/// EBADF, before any readiness is looked at. The sets are written back only
/// when something is ready, since a select that waits is tried again with
/// the same sets; `clear` empties them when its time runs out. Nothing here
/// has an exceptional condition, so that set always comes back empty.
pub fn select(guest: &mut Guest, nfds: u64, sets: [u64; 3]) -> u64 {
    let nfds = match select_count(nfds, MAX_FDS as u64) {
        Ok(n) => n,
        Err(e) => return errno::fail(e),
    };
    let bytes = set_bytes(nfds);
    let mut read: [Option<Vec<u8>>; 3] = [None, None, None];
    for (slot, &at) in read.iter_mut().zip(sets.iter()).filter(|(_, &at)| at != 0) {
        match guest.read(at, bytes) {
            Some(set) => *slot = Some(set),
            None => return errno::fail(errno::EFAULT),
        }
    }
    let named: Vec<&[u8]> = read.iter().flatten().map(Vec::as_slice).collect();
    let open = |fd: u64| guest.fds.get(fd as usize).is_some_and(|f| f.is_open());
    if first_closed(&named, nfds, open).is_some() {
        return errno::fail(errno::EBADF);
    }
    let mut hits = 0u64;
    for (set, want) in read.iter_mut().zip([POLLIN, POLLOUT]) {
        if let Some(set) = set {
            hits += narrow(set, nfds, |fd| ready(guest, fd), want);
        }
    }
    if hits == 0 {
        return errno::ok(0);
    }
    if let Some(except) = read[2].as_mut() {
        except.fill(0);
    }
    for (set, &at) in read.iter().zip(sets.iter()) {
        if let Some(set) = set {
            if guest.write(at, set) < 0 {
                return errno::fail(errno::EFAULT);
            }
        }
    }
    errno::ok(hits)
}

/// The sets as a select whose time ran out leaves them: empty.
pub fn clear(guest: &mut Guest, nfds: u64, sets: [u64; 3]) {
    let Ok(nfds) = select_count(nfds, MAX_FDS as u64) else {
        return;
    };
    let empty = vec![0u8; set_bytes(nfds)];
    for at in sets.into_iter().filter(|&at| at != 0) {
        let _ = guest.write(at, &empty);
    }
}
