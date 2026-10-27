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

//! A new pipe's buffer and both its descriptors: pipe2's (make.rs), and the
//! display server's, which reads a guest's selection through one.

use crate::linux::abi::errno;
use crate::linux::file::install;
use crate::linux::guest::slots::pick;
use crate::linux::guest::{Fd, Guest};

/// ENFILE: no slot left in the family's table of pipes.
const ENFILE: i64 = 23;

/// The buffer, the read end and the write end, the buffer not yet set up;
/// or the errno, with no descriptor left behind.
pub(super) fn take(guest: &mut Guest) -> Result<(usize, u64, u64), i64> {
    /*
     * A buffer no end names anywhere in the family is taken again before the
     * table grows; without that, every pipe2 a guest made, and every one it
     * was refused, grew this capsule's memory for good.
     */
    let lent = guest.pipe_ends.len() == guest.pipes.len();
    let free = |i: usize| lent && guest.pipe_ends.get(i) == Some(&(false, false));
    let buffer = pick(guest.pipes.len(), free).ok_or(ENFILE)?;
    let read_end = install(guest, Fd::pipe(buffer as u32, false)).ok_or(errno::EMFILE)?;
    let Some(write_end) = install(guest, Fd::pipe(buffer as u32, true)) else {
        super::ends::release(guest, read_end);
        return Err(errno::EMFILE);
    };
    Ok((buffer, read_end, write_end))
}

/// A pipe made whole, both ends given `flags`: the buffer, the read end and
/// the write end, or None when the guest has no room for one.
pub fn pipe_pair(guest: &mut Guest, flags: u64) -> Option<(usize, u64, u64)> {
    let (buffer, read_end, write_end) = take(guest).ok()?;
    super::ends::set_up(guest, buffer, [read_end, write_end], flags);
    Some((buffer, read_end, write_end))
}
