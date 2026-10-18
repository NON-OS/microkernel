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

//! Which buffer a pipe descriptor names, and which end of it it is.

use crate::linux::guest::{Guest, Kind};

pub fn end_of(guest: &Guest, fd: u64) -> Option<(usize, bool)> {
    let entry = guest.fds.get(fd as usize)?;
    if entry.kind != Kind::Pipe {
        return None;
    }
    let slot = entry.handle as usize;
    match slot < guest.pipes.len() {
        true => Some((slot, entry.writable)),
        false => None,
    }
}

/// Whether the other end of pipe `slot` is open anywhere in the family, seen
/// from the end that is `writable` or not. A pipe made during this answer is
/// not in the family's note yet, and both its ends are open.
pub fn other_end_open(guest: &Guest, slot: usize, writable: bool) -> bool {
    guest.pipe_ends.get(slot).is_none_or(|&(read, write)| if writable { read } else { write })
}
