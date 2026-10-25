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

//! Which counter an eventfd names, and what poll and epoll see of it.

use crate::linux::guest::{Guest, Kind};

use super::open::MOST;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLNVAL: u16 = 0x020;

/// The counter `fd` names, if it is an eventfd.
pub fn slot_of(guest: &Guest, fd: u64) -> Option<usize> {
    let entry = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Event)?;
    let slot = entry.handle as usize;
    (slot < guest.events.len()).then_some(slot)
}

/// Readable while the count is above zero, writable while one more fits.
pub fn bits(guest: &Guest, fd: u64) -> u16 {
    let Some(slot) = slot_of(guest, fd) else {
        return POLLNVAL;
    };
    let count = guest.events[slot].count;
    let readable = if count > 0 { POLLIN } else { 0 };
    readable | if count < MOST { POLLOUT } else { 0 }
}
