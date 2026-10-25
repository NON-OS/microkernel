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

//! `pipe2`.

use crate::linux::abi::errno;
use crate::linux::guest::slots::pick;
use crate::linux::guest::{Fd, Guest};

use crate::linux::file::flags::{O_CLOEXEC, O_NONBLOCK};
use crate::linux::file::install;

/// ENFILE: no slot left in the family's table of pipes.
const ENFILE: i64 = 23;

/// In Linux's order: an unknown flag is EINVAL (O_DIRECT's packet pipes and
/// notification pipes are not offered, so they are refused rather than
/// served as byte pipes); both descriptors are taken before either is
/// given out, so EMFILE leaves neither behind; and a pair that cannot be
/// written back is EFAULT with both closed again, as do_pipe2 does.
pub fn pipe2(guest: &mut Guest, out: u64, flags: u64) -> u64 {
    if flags & !(O_CLOEXEC | O_NONBLOCK) != 0 {
        return errno::fail(errno::EINVAL);
    }
    /*
     * A buffer no end names anywhere in the family is taken again before the
     * table grows; without that, every pipe2 a guest made, and every one it
     * was refused, grew this capsule's memory for good.
     */
    let lent = guest.pipe_ends.len() == guest.pipes.len();
    let free = |i: usize| lent && guest.pipe_ends.get(i) == Some(&(false, false));
    let Some(buffer) = pick(guest.pipes.len(), free) else {
        return errno::fail(ENFILE);
    };
    let Some(read_end) = install(guest, Fd::pipe(buffer as u32, false)) else {
        return errno::fail(errno::EMFILE);
    };
    let Some(write_end) = install(guest, Fd::pipe(buffer as u32, true)) else {
        super::ends::release(guest, read_end);
        return errno::fail(errno::EMFILE);
    };
    let mut pair = [0u8; 8];
    pair[..4].copy_from_slice(&(read_end as u32).to_le_bytes());
    pair[4..].copy_from_slice(&(write_end as u32).to_le_bytes());
    if guest.write(out, &pair) < 8 {
        super::ends::release(guest, read_end);
        super::ends::release(guest, write_end);
        return errno::fail(errno::EFAULT);
    }
    super::ends::set_up(guest, buffer, [read_end, write_end], flags);
    errno::ok(0)
}
