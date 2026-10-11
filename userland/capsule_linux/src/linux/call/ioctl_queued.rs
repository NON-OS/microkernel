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

//! FIONREAD: how many bytes a read would find now.

use crate::linux::guest::{Guest, Kind};

/// Bytes a read would find now: what a pipe holds, from either end, as
/// Linux's pipe_ioctl counts it; what is left of a file past its offset;
/// what was typed at the terminal; and nothing for the console off one,
/// which is a pipe at its end. None for a descriptor Linux answers ENOTTY.
pub(super) fn waiting(guest: &Guest, fd: u64, on_terminal: bool) -> Option<u64> {
    let entry = guest.fds.get(fd as usize)?;
    match entry.kind {
        _ if on_terminal => Some(crate::linux::console::queued()),
        Kind::Stdin => Some(0),
        Kind::Pipe => guest.pipes.get(entry.handle as usize).map(|p| p.len() as u64),
        Kind::File => Some(entry.size.saturating_sub(entry.offset)),
        _ => None,
    }
}
