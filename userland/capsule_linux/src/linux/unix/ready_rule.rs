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
//! The display socket's poll bits. Pure, so the host proofs hold it.
//!
//! It read as always readable, and GTK, whose display source outranks its
//! paint, found the display ready, read nothing, and polled again forever:
//! zathura and Mousepad acked their first configure and never drew.

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;

/// Readable while the server has said what the client has not read,
/// writable while a write has room. One not connected reads as before.
pub fn display_bits(connected: bool, unread: bool, room: bool) -> u16 {
    if !connected {
        return POLLIN | POLLOUT;
    }
    (if unread { POLLIN } else { 0 }) | if room { POLLOUT } else { 0 }
}
