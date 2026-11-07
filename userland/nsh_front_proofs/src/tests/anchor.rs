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

//! The anchor file: 32 raw bytes, or the 64 hex digits nsh-roles prints,
//! with or without one newline; nothing else.

use crate::anchor::{anchor, ANCHOR_FILE_MAX};

const ID: [u8; 32] = [
    0x00, 0x01, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe, 0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78,
];

fn hex(upper: bool) -> String {
    ID.iter().map(|b| if upper { format!("{b:02X}") } else { format!("{b:02x}") }).collect()
}

#[test]
fn raw_and_hex_anchors_are_the_same_key_id() {
    assert_eq!(anchor(&ID), Some(ID));
    assert_eq!(anchor(hex(false).as_bytes()), Some(ID));
    assert_eq!(anchor(format!("{}\n", hex(false)).as_bytes()), Some(ID));
    assert_eq!(anchor(hex(true).as_bytes()), Some(ID));
    assert_eq!(ANCHOR_FILE_MAX, 65);
}

#[test]
fn anything_else_is_no_anchor() {
    let h = hex(false);
    for bad in [
        &b""[..],
        &ID[..31],
        &h.as_bytes()[..63],
        format!("{h}\n\n").as_bytes(),
        format!("{h} ").as_bytes(),
        format!("0x{}", &h[2..]).as_bytes(),
        format!("{}g", &h[..63]).as_bytes(),
        format!("{h}\r\n").as_bytes(),
    ] {
        assert_eq!(anchor(bad), None, "{bad:?}");
    }
}
