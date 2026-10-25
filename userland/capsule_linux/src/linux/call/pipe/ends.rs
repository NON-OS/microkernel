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

//! A new pipe's two descriptors once both are given out, and one taken
//! back when the pipe is not made after all (make.rs).

use alloc::vec::Vec;

use crate::linux::file::flags::{O_CLOEXEC, O_NONBLOCK};
use crate::linux::guest::{Fd, Guest, Kind};

/// Both ends get the flags pipe2 was asked for, and `buffer` starts empty
/// with both its ends open.
pub(super) fn set_up(guest: &mut Guest, buffer: usize, ends: [u64; 2], flags: u64) {
    for end in ends {
        if let Some(fd) = guest.fds.get_mut(end as usize) {
            fd.cloexec = flags & O_CLOEXEC != 0;
            fd.nonblock = flags & O_NONBLOCK != 0;
        }
    }
    match guest.pipes.get_mut(buffer) {
        Some(old) => *old = Vec::new(),
        None => guest.pipes.push(Vec::new()),
    }
    if let Some(ends) = guest.pipe_ends.get_mut(buffer) {
        *ends = (true, true);
    }
}

/// A descriptor taken for a pipe that is not being made after all.
pub(super) fn release(guest: &mut Guest, fd: u64) {
    if let Some(slot) = guest.fds.get_mut(fd as usize) {
        *slot = Fd::empty(Kind::Free);
    }
}
