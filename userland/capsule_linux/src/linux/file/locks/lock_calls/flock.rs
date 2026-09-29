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

/* flock: a lock on an open file description. */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::super::desc;
use super::super::lock::{self, Lock, Owner};
use super::purge::purge;

const LOCK_SH: u64 = 1;

const LOCK_EX: u64 = 2;

const LOCK_NB: u64 = 4;

const LOCK_UN: u64 = 8;

/* The answer that means "not yet": parked, not replied. */
pub const WAIT: u64 = u64::MAX - 1000;

pub(super) fn file_of(guest: &Guest, fd: u64) -> Result<(&Fd, u32), u64> {
    let f = guest.fds.get(fd as usize).filter(|f| f.is_open()).ok_or(errno::fail(errno::EBADF))?;
    let d = desc::of(f).ok_or(errno::fail(errno::EINVAL))?;
    Ok((f, d))
}

pub fn flock(guest: &Guest, fd: u64, op: u64) -> u64 {
    let (f, d) = match file_of(guest, fd) {
        Ok(x) => x,
        Err(e) => return e,
    };
    let whole = |write| Lock {
        file: f.path.clone(),
        owner: Owner::Flock(d),
        write,
        start: 0,
        end: u64::MAX,
    };
    match op & !LOCK_NB {
        LOCK_UN => {
            lock::apply(&whole(false), false);
            errno::ok(0)
        }
        LOCK_SH | LOCK_EX => {
            purge();
            match lock::take(whole(op & LOCK_EX != 0)) {
                Ok(()) => errno::ok(0),
                Err(_) if op & LOCK_NB != 0 => errno::fail(errno::EAGAIN),
                Err(_) => WAIT,
            }
        }
        _ => errno::fail(errno::EINVAL),
    }
}
