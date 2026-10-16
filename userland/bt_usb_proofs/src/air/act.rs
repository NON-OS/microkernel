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

//! The acts a test scripts for the air.

/// What happens once the host has had nothing to do for a while.
pub enum Act {
    /// A notification from the peripheral: handle and value.
    Notify(u16, Vec<u8>),
    /// A frame on the LE signaling channel.
    Signal(Vec<u8>),
    /// The link goes down with this reason.
    Drop(u8),
    /// Any other event from the controller; or many at once, a flood.
    Event(Vec<u8>),
    Events(Vec<Vec<u8>>),
    /// ACL packets exactly as given, on any handle.
    Acl(Vec<Vec<u8>>),
    /// An event and a signaling frame from the peer, taken in one turn.
    Together(Vec<u8>, Vec<u8>),
    /// The controller starts lying from here on.
    Lie(fn(&mut super::Lies)),
}

/// A Handle Value Notification (Vol 3, Part F, 3.4.7.1).
pub fn notification(handle: u16, value: &[u8]) -> Vec<u8> {
    [&[0x1b][..], &handle.to_le_bytes(), value].concat()
}
