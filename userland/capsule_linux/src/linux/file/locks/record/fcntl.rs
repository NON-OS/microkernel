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

/* fcntl's record locks: F_GETLK, F_SETLK and F_SETLKW, and the OFD forms. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::desc;
use super::super::lock::{self};
use super::super::lock_calls::{purge, WAIT};
use super::cmds::{F_GETLK, F_OFD_GETLK, F_OFD_SETLKW, F_RDLCK, F_SETLKW, F_UNLCK, F_WRLCK};
use super::range::report;
use super::want::wanted;

pub fn fcntl_lock(guest: &Guest, fd: u64, cmd: u64, arg: u64) -> u64 {
    let Some(f) = guest.fds.get(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    let (want, kind) = match wanted(guest, f, cmd, arg) {
        Ok(w) => w,
        Err(e) => return e,
    };
    purge();
    match (cmd, kind) {
        (F_GETLK | F_OFD_GETLK, F_RDLCK | F_WRLCK) => report(guest, arg, lock::blocker(&want)),
        (_, F_UNLCK) if !matches!(cmd, F_GETLK | F_OFD_GETLK) => {
            lock::apply(&want, false);
            errno::ok(0)
        }
        /* Linux wants the descriptor open for what the lock is for. */
        (_, F_WRLCK) if !f.writable => errno::fail(errno::EBADF),
        (_, F_RDLCK) if !desc::reads(f) => errno::fail(errno::EBADF),
        (_, F_RDLCK | F_WRLCK) => match lock::take(want) {
            Ok(()) => errno::ok(0),
            Err(_) if matches!(cmd, F_SETLKW | F_OFD_SETLKW) => WAIT,
            Err(_) => errno::fail(errno::EAGAIN),
        },
        _ => errno::fail(errno::EINVAL),
    }
}
