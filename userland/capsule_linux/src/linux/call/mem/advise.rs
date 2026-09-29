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

//! `madvise`. Advice a Linux program counts on is carried out, advice that
//! changes nothing a program can read is taken as the hint it is, and advice
//! this capsule cannot carry out is refused, never answered with a success.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, PAGE};

use super::advise_drop::{drop_pages, parts};

const DONTNEED: u64 = 4;
const DONTNEED_LOCKED: u64 = 24;
/* Linux 6.1's hints, none of which changes what a program reads. DOFORK and
 * KEEPONFORK undo advice this capsule never takes. */
const HINTS: [u64; 18] = [0, 1, 2, 3, 8, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 25];
/* REMOVE, DONTFORK and WIPEONFORK would change what a program reads or what
 * a child inherits, and this capsule does neither. */
const REFUSED: [u64; 3] = [9, 10, 18];
/* HWPOISON and SOFT_OFFLINE, which Linux keeps for a privileged caller. */
const PRIVILEGED: [u64; 2] = [100, 101];

/// In Linux's order: unknown advice or a misaligned address is EINVAL, an
/// empty span is 0, privileged advice EPERM, a span with a page not mapped
/// ENOMEM, and then the advice itself.
pub fn madvise(guest: &mut Guest, addr: u64, len: u64, advice: u64) -> u64 {
    let dontneed = advice == DONTNEED || advice == DONTNEED_LOCKED;
    let known = dontneed || [&HINTS[..], &REFUSED, &PRIVILEGED].iter().any(|s| s.contains(&advice));
    let end = len.checked_add(PAGE - 1).map(|l| l & !(PAGE - 1)).and_then(|l| addr.checked_add(l));
    let Some(end) = end.filter(|_| known && addr % PAGE == 0) else {
        return errno::fail(errno::EINVAL);
    };
    if end == addr {
        return errno::ok(0);
    }
    if PRIVILEGED.contains(&advice) {
        return errno::fail(errno::EPERM);
    }
    let Some(parts) = parts(guest, addr, end) else {
        return errno::fail(errno::ENOMEM);
    };
    match advice {
        a if REFUSED.contains(&a) => errno::fail(errno::EINVAL),
        _ if dontneed => drop_pages(guest, &parts),
        _ => errno::ok(0),
    }
}
