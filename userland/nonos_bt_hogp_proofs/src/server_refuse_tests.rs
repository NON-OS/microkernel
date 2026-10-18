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
//! Requests the host's database refuses, each with the Error Response the
//! cited section asks for (Vol 3, Part F, 3.4.1.1).

use nonos_bt_hogp::server::serve;

fn ask(pdu: &[u8]) -> Vec<u8> {
    serve(pdu[0], &pdu[1..])
}

#[test]
fn bad_ranges_are_an_invalid_handle_on_the_start() {
    for op in [0x04, 0x08, 0x10] {
        let tail: &[u8] = if op == 0x04 { &[] } else { &[0x00, 0x28] };
        let zero = [&[op, 0x00, 0x00, 0xff, 0xff][..], tail].concat();
        assert_eq!(ask(&zero), [0x01, op, 0, 0, 0x01]);
        let back = [&[op, 0x05, 0x00, 0x04, 0x00][..], tail].concat();
        assert_eq!(ask(&back), [0x01, op, 5, 0, 0x01]);
    }
    assert_eq!(ask(&[0x0a, 0x09, 0x00]), [0x01, 0x0a, 9, 0, 0x01]);
}

#[test]
fn only_services_group_and_nothing_is_written() {
    // Characteristic is not a grouping type; there is no secondary service.
    assert_eq!(ask(&[0x10, 0x01, 0x00, 0xff, 0xff, 0x03, 0x28]), [0x01, 0x10, 1, 0, 0x10]);
    assert_eq!(ask(&[0x10, 0x01, 0x00, 0xff, 0xff, 0x01, 0x28]), [0x01, 0x10, 1, 0, 0x0a]);
    // Write Not Permitted on an attribute, Invalid Handle on none.
    assert_eq!(ask(&[0x12, 0x03, 0x00, b'x']), [0x01, 0x12, 3, 0, 0x03]);
    assert_eq!(ask(&[0x12, 0x20, 0x00, b'x']), [0x01, 0x12, 0x20, 0, 0x01]);
}

#[test]
fn the_mtu_answer_is_this_hosts_and_the_rest_is_not_supported() {
    // 247, the same Rx MTU the host's client offers.
    assert_eq!(ask(&[0x02, 0x00, 0x02]), [0x03, 247, 0]);
    // Malformed: Invalid PDU on handle 0.
    assert_eq!(ask(&[0x02, 0x00]), [0x01, 0x02, 0, 0, 0x04]);
    assert_eq!(ask(&[0x08, 0x01, 0x00, 0xff, 0xff, 0x00]), [0x01, 0x08, 0, 0, 0x04]);
    assert_eq!(ask(&[0x0a, 0x03]), [0x01, 0x0a, 0, 0, 0x04]);
    // Read Blob and Prepare Write: Request Not Supported.
    for op in [0x0c, 0x16] {
        assert_eq!(ask(&[op, 0x03, 0x00, 0x00, 0x00]), [0x01, op, 0, 0, 0x06]);
    }
}
