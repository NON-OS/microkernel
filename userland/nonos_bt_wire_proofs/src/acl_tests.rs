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

//! ACL packets (Vol 4, Part E, 5.4.2).

use nonos_bt_wire::hci::{acl, parse_acl, Boundary, HANDLE_MAX};
use nonos_bt_wire::WireError;

#[test]
fn handle_and_flags_share_the_first_word() {
    let pkt = acl(0x0040, Boundary::FirstNonFlushable, &[1, 2, 3]).unwrap();
    assert_eq!(pkt, [0x40, 0x00, 3, 0, 1, 2, 3]);
    let pkt = acl(0x0123, Boundary::Continuing, &[9]).unwrap();
    assert_eq!(pkt, [0x23, 0x11, 1, 0, 9]);
    let (h, data) = parse_acl(&[0x40, 0x20, 2, 0, 7, 8]).ok().unwrap();
    assert_eq!(
        (h.handle, h.boundary, h.len, data),
        (0x40, Boundary::FirstFlushable, 2, &[7u8, 8][..])
    );
}

#[test]
fn reserved_handles_broadcast_and_bad_lengths_are_refused() {
    assert_eq!(acl(HANDLE_MAX + 1, Boundary::Complete, &[]).err(), Some(WireError::Reserved));
    assert_eq!(parse_acl(&[0x00, 0x0f, 0, 0]).err(), Some(WireError::Reserved));
    assert_eq!(parse_acl(&[0x40, 0x40, 0, 0]).err(), Some(WireError::Reserved));
    assert_eq!(parse_acl(&[0x40, 0x00, 2, 0, 7]).err(), Some(WireError::Short));
    assert_eq!(parse_acl(&[0x40, 0x00, 0, 0, 7]).err(), Some(WireError::Length));
    assert_eq!(acl(1, Boundary::Complete, &vec![0; 65536]).err(), Some(WireError::TooLong));
}
