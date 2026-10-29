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

//! The display socket is readable only while the client has something to
//! read. Always readable, a GTK client found it ready, read nothing (EAGAIN),
//! and polled again forever ahead of its own paint, so it never drew.

use crate::display_ready::display_bits;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;

#[test]
fn nothing_unread_is_not_readable() {
    assert_eq!(display_bits(true, false, true), POLLOUT);
}

#[test]
fn unread_bytes_are_readable() {
    assert_eq!(display_bits(true, true, true), POLLIN | POLLOUT);
}

#[test]
fn a_full_request_buffer_is_not_writable() {
    assert_eq!(display_bits(true, true, false), POLLIN);
    assert_eq!(display_bits(true, false, false), 0);
}

#[test]
fn a_socket_not_yet_connected_reads_as_it_did() {
    assert_eq!(display_bits(false, false, false), POLLIN | POLLOUT);
}
