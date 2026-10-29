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
//! What poll says of the display socket, and the input taken in before it
//! is asked (ready_rule.rs).

use crate::linux::guest::{Guest, Kind};

use super::ready_rule::display_bits;
use super::sock::connected;

/// poll's bits for display socket `fd`.
pub fn bits(guest: &Guest, fd: u64) -> u16 {
    let d = &guest.display;
    display_bits(connected(guest, fd), !d.to_client.is_empty(), d.room() > 0)
}

/// The input that has come for a connected display, taken in, so a wait on
/// it sees what a read would give.
pub fn pump(guest: &mut Guest) {
    if guest.fds.iter().any(|f| f.kind == Kind::Unix && f.writable) {
        crate::linux::wayland::pump(guest);
    }
}
