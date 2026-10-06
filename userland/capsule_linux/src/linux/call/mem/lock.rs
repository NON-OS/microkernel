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
//! `mlock`, `mlock2`, `munlock`, `mlockall` and `munlockall`.
//!
//! Locking keeps pages resident. Every page a guest holds here is resident
//! from the moment it is mapped until it is unmapped, and none is ever paged
//! out, so every page is already as locked as Linux can make it. What is left
//! of each call is Linux's checking of its arguments, answered the same way:
//! the span must be held, and the flags known. RLIMIT_MEMLOCK is reported
//! unlimited, so no lock is refused for its size.

use crate::linux::abi::errno;
use crate::linux::guest::{page_down, page_up, Guest, PAGE};

const MLOCK_ONFAULT: u64 = 1;
const MCL_CURRENT: u64 = 1;
const MCL_FUTURE: u64 = 2;
const MCL_ONFAULT: u64 = 4;

/// `mlock` and `munlock` alike: the address is rounded down, the length up,
/// and a span with a page the guest does not hold is ENOMEM.
pub fn mlock(guest: &Guest, addr: u64, len: u64) -> u64 {
    let start = page_down(addr);
    let Some(end) = addr.checked_add(len).filter(|e| *e <= u64::MAX - PAGE) else {
        return errno::fail(errno::EINVAL);
    };
    let end = page_up(end);
    if end == start {
        return errno::ok(0);
    }
    if guest.mapped_from(start) < end - start {
        return errno::fail(errno::ENOMEM);
    }
    errno::ok(0)
}

pub fn mlock2(guest: &Guest, addr: u64, len: u64, flags: u64) -> u64 {
    if flags & !MLOCK_ONFAULT != 0 {
        return errno::fail(errno::EINVAL);
    }
    mlock(guest, addr, len)
}

/// Linux refuses no flags, unknown flags, and MCL_ONFAULT on its own.
pub fn mlockall(flags: u64) -> u64 {
    let known = MCL_CURRENT | MCL_FUTURE | MCL_ONFAULT;
    if flags == 0 || flags & !known != 0 || flags == MCL_ONFAULT {
        return errno::fail(errno::EINVAL);
    }
    errno::ok(0)
}
