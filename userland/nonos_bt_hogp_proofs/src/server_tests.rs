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
//! The host's own database as a GATT client discovers it (Vol 3, Part G,
//! 4.4 to 4.8), each response laid out by hand from Vol 3, Part F, 3.4.

use nonos_bt_hogp::server::serve;

fn ask(pdu: &[u8]) -> Vec<u8> {
    serve(pdu[0], &pdu[1..])
}

#[test]
fn the_primary_services_and_their_ends() {
    let gap_gatt = [0x11, 6, 1, 0, 5, 0, 0x00, 0x18, 6, 0, 6, 0, 0x01, 0x18];
    assert_eq!(ask(&[0x10, 0x01, 0x00, 0xff, 0xff, 0x00, 0x28]), gap_gatt);
    assert_eq!(ask(&[0x10, 0x07, 0x00, 0xff, 0xff, 0x00, 0x28]), [0x01, 0x10, 7, 0, 0x0a]);
    assert_eq!(ask(&[0x06, 0x01, 0x00, 0xff, 0xff, 0x00, 0x28, 0x00, 0x18]), [0x07, 1, 0, 5, 0]);
    let hid = [0x06, 0x01, 0x00, 0xff, 0xff, 0x00, 0x28, 0x12, 0x18];
    assert_eq!(ask(&hid), [0x01, 0x06, 1, 0, 0x0a]);
}

#[test]
fn the_characteristics_and_their_values() {
    let decls = [0x09, 7, 2, 0, 0x02, 3, 0, 0x00, 0x2a, 4, 0, 0x02, 5, 0, 0x01, 0x2a];
    assert_eq!(ask(&[0x08, 0x01, 0x00, 0x05, 0x00, 0x03, 0x28]), decls);
    let name = [0x09, 7, 3, 0, b'N', b'O', b'N', b'O', b'S'];
    assert_eq!(ask(&[0x08, 0x01, 0x00, 0xff, 0xff, 0x00, 0x2a]), name);
    // The same type as a 128-bit UUID in the base range.
    let mut long = vec![0x08, 0x01, 0x00, 0xff, 0xff];
    long.extend_from_slice(&nonos_bt_wire::att::BASE_UUID);
    long[17..19].copy_from_slice(&[0x00, 0x2a]);
    assert_eq!(ask(&long), name);
    assert_eq!(ask(&[0x0a, 0x05, 0x00]), [0x0b, 0x80, 0x00]);
    assert_eq!(ask(&[0x0a, 0x03, 0x00]), [0x0b, b'N', b'O', b'N', b'O', b'S']);
}

#[test]
fn find_information_stops_where_the_mtu_does() {
    // Five handle and type pairs fill 22 of the 23 octets; the sixth waits
    // for the next request.
    let first = [
        0x05, 0x01, 1, 0, 0x00, 0x28, 2, 0, 0x03, 0x28, 3, 0, 0x00, 0x2a, 4, 0, 0x03, 0x28, 5, 0,
        0x01, 0x2a,
    ];
    assert_eq!(ask(&[0x04, 0x01, 0x00, 0xff, 0xff]), first);
    assert_eq!(ask(&[0x04, 0x06, 0x00, 0xff, 0xff]), [0x05, 0x01, 6, 0, 0x00, 0x28]);
    assert_eq!(ask(&[0x04, 0x07, 0x00, 0xff, 0xff]), [0x01, 0x04, 7, 0, 0x0a]);
}
