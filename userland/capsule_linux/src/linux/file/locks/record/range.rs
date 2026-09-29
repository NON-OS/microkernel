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

/* The range a struct flock names, and the lock reported back in it. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::lock::Lock;
use super::super::lock_calls::owners_ns;
use super::cmds::{FLOCK, F_RDLCK, F_UNLCK, F_WRLCK};

/*
 * [from, to) for a start and a length from `base`; a negative length
 * counts back from the start, and zero means "to the end".
 */
pub(super) fn range(base: i64, start: i64, len: i64) -> Option<(u64, u64)> {
    let at = base.checked_add(start)?;
    let (from, to) = match len {
        0 => (at, i64::MAX),
        l if l > 0 => (at, at.checked_add(l)?),
        l => (at.checked_add(l)?, at),
    };
    (from >= 0).then(|| (from as u64, if to == i64::MAX { u64::MAX } else { to as u64 }))
}

/* F_GETLK: the lock in the way, or F_UNLCK when there is none. */
pub(super) fn report(guest: &Guest, arg: u64, found: Option<Lock>) -> u64 {
    let mut out = [0u8; FLOCK];
    match found {
        None => out[0..2].copy_from_slice(&F_UNLCK.to_le_bytes()),
        Some(l) => {
            let kind = if l.write { F_WRLCK } else { F_RDLCK };
            let len = if l.end == u64::MAX { 0 } else { l.end - l.start };
            out[0..2].copy_from_slice(&kind.to_le_bytes());
            out[8..16].copy_from_slice(&(l.start as i64).to_le_bytes());
            out[16..24].copy_from_slice(&(len as i64).to_le_bytes());
            out[24..28].copy_from_slice(&owners_ns(l.owner).to_le_bytes());
        }
    }
    match guest.write(arg, &out) {
        n if n < FLOCK as i64 => errno::fail(errno::EFAULT),
        _ => errno::ok(0),
    }
}
