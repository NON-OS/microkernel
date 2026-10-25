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

//! The Three-wire UART's octets laid out by hand from Vol 4, Part D: the
//! SYNC a starting side sends, a header with every field set, and SLIP
//! with and without out-of-frame flow control.

use nonos_bt_hci::uart::h5::header::Header;
use nonos_bt_hci::uart::h5::slip::encode;
use nonos_bt_hci::uart::h5::H5;

#[test]
fn a_starting_side_sends_sync_as_an_unreliable_link_control_packet() {
    // Header: seq 0, ack 0, no check, unreliable; type 15, length 2; then
    // the checksum 0xff - 0x2f. Payload 01 7E. Between two 0xC0.
    let mut h5 = H5::default();
    assert_eq!(h5.poll_out(0, 250), Some(vec![0xc0, 0x00, 0x2f, 0x00, 0xd0, 0x01, 0x7e, 0xc0]));
    assert_eq!(h5.poll_out(249, 250), None, "not again before the resend time");
    assert!(h5.poll_out(250, 250).is_some());
}

#[test]
fn a_header_lays_out_every_field() {
    let h = Header { seq: 3, ack: 5, dic: true, reliable: true, kind: 4, len: 0x123 };
    // 3 | 5 << 3 | 0x40 | 0x80; 4 | 0x3 << 4; 0x12; 0xff - (0xeb + 0x34 + 0x12) mod 256.
    assert_eq!(h.encode(), [0xeb, 0x34, 0x12, 0xce]);
    assert_eq!(Header::parse([0xeb, 0x34, 0x12, 0xce]), Some(h));
    assert_eq!(Header::parse([0xeb, 0x34, 0x12, 0xcf]), None, "checksum off by one");
}

#[test]
fn every_single_bit_flip_in_a_header_is_refused() {
    let good = Header { seq: 6, ack: 1, dic: false, reliable: true, kind: 2, len: 27 }.encode();
    for bit in 0..32 {
        let mut h = good;
        h[bit / 8] ^= 1 << (bit % 8);
        assert_eq!(Header::parse(h), None, "bit {bit}");
    }
}

#[test]
fn slip_escapes_the_delimiter_and_escape_and_with_oof_xon_xoff() {
    let p = [0xc0, 0xdb, 0x11, 0x13, 0x00];
    assert_eq!(encode(&p, false), [0xc0, 0xdb, 0xdc, 0xdb, 0xdd, 0x11, 0x13, 0x00, 0xc0]);
    let oof = [0xc0, 0xdb, 0xdc, 0xdb, 0xdd, 0xdb, 0xde, 0xdb, 0xdf, 0x00, 0xc0];
    assert_eq!(encode(&p, true), oof);
}
