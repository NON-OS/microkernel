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

/* The lock a struct flock asks for. */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::super::desc;
use super::super::lock::{Lock, Owner};
use super::cmds::{FLOCK, F_OFD_GETLK, F_OFD_SETLK, F_OFD_SETLKW, F_WRLCK};
use super::range::range;

/*
 * The lock the struct flock at `arg` asks for on `f`, and its l_type: the
 * range from l_whence, l_start and l_len, owned by the process or, for the
 * OFD commands, by the open file description.
 */
pub(super) fn wanted(guest: &Guest, f: &Fd, cmd: u64, arg: u64) -> Result<(Lock, i16), u64> {
    let d = desc::of(f).ok_or_else(|| errno::fail(errno::EINVAL))?;
    let raw = guest.read(arg, FLOCK).ok_or_else(|| errno::fail(errno::EFAULT))?;
    let short = |at: usize| i16::from_le_bytes([raw[at], raw[at + 1]]);
    let long = |at: usize| i64::from_le_bytes(raw[at..at + 8].try_into().unwrap_or([0; 8]));
    let (kind, whence, start, len) = (short(0), short(2), long(8), long(16));
    let ofd = matches!(cmd, F_OFD_GETLK | F_OFD_SETLK | F_OFD_SETLKW);
    /* An OFD lock must say l_pid 0. */
    if ofd && long(24) as i32 != 0 {
        return Err(errno::fail(errno::EINVAL));
    }
    let base = match whence {
        0 => 0,
        1 => desc::pos(f) as i64,
        2 => f.size.max(f.pending.len() as u64) as i64,
        _ => return Err(errno::fail(errno::EINVAL)),
    };
    let (from, to) = range(base, start, len).ok_or_else(|| errno::fail(errno::EINVAL))?;
    let owner = if ofd { Owner::Ofd(d) } else { Owner::Posix(guest.pid) };
    Ok((Lock { file: f.path.clone(), owner, write: kind == F_WRLCK, start: from, end: to }, kind))
}
