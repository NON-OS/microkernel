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

//! `MkTtySet` and `MkTtyQuery`: how a program learns whether it is writing
//! to a person or to a pipe. A launcher that renders a child's output
//! knows which of the child's streams reach its screen; the child asks
//! before choosing colour, columns and a pager.

use super::errnos::{ERRNO_INVAL, ERRNO_NOTTY, ERRNO_PERM};
use super::tty_table::{self, Tty, STREAMS_ALL};
use crate::process::{current_pid, get_parent_pid};

/// `MkTtySet(pid, streams, cols, rows)`. Only the parent of `pid` may say
/// what its streams are on, the same rule that lets it drain them; anyone
/// else would be telling a stranger's program it has a screen.
pub fn sys_tty_set(pid: u64, streams: u64, cols: u64, rows: u64) -> i64 {
    let fits = |v: u64| v <= u16::MAX as u64;
    if pid == 0 || pid > u32::MAX as u64 || streams > STREAMS_ALL as u64 || !fits(cols) || !fits(rows)
    {
        return ERRNO_INVAL;
    }
    let caller = current_pid().unwrap_or(0);
    if caller == 0 || get_parent_pid(pid as u32) != Some(caller) {
        return ERRNO_PERM;
    }
    tty_table::set(pid as u32, Tty { streams: streams as u8, cols: cols as u16, rows: rows as u16 });
    0
}

/// `MkTtyQuery(fd)`. For the caller's stdin, stdout or stderr on a
/// terminal, its size as `rows << 16 | cols`; ENOTTY for anything else.
pub fn sys_tty_query(fd: u64) -> i64 {
    if fd > 2 {
        return ERRNO_NOTTY;
    }
    let Some(pid) = current_pid() else { return ERRNO_NOTTY };
    match tty_table::get(pid) {
        Some(t) if t.streams & (1 << fd) != 0 => ((t.rows as i64) << 16) | t.cols as i64,
        _ => ERRNO_NOTTY,
    }
}
