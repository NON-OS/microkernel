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

use super::identity::decode_identity;

fn body(model: &[u8], serial: &[u8], medium: u8) -> [u8; 64] {
    let mut b = [0u8; 64];
    (b[0], b[1], b[2]) = (model.len() as u8, serial.len() as u8, medium);
    b[4..4 + model.len()].copy_from_slice(model);
    b[44..44 + serial.len()].copy_from_slice(serial);
    b
}

#[test]
fn a_well_formed_body_names_the_part() {
    let id = decode_identity(&body(b"SK hynix HAG4a2", b"01020304", 1)).expect("taken");
    assert_eq!(
        (id.model(), id.serial(), id.medium),
        (&b"SK hynix HAG4a2"[..], &b"01020304"[..], 1)
    );
    let blank = decode_identity(&body(b"", b"", 0)).expect("no name is still a body");
    assert!(blank.model().is_empty() && blank.serial().is_empty());
    let full = decode_identity(&body(&[b'M'; 40], &[b'S'; 20], 2)).expect("both fields full");
    assert_eq!((full.model().len(), full.serial().len()), (40, 20));
}

#[test]
fn a_wrong_size_a_length_past_its_field_or_a_set_reserved_byte_is_refused() {
    for n in [0, 4, 63, 65] {
        let mut v = body(b"x", b"y", 1).to_vec();
        v.resize(n, 0);
        assert!(decode_identity(&v).is_none(), "{n} bytes");
    }
    let mut b = body(b"x", b"y", 1);
    b[0] = 41;
    assert!(decode_identity(&b).is_none());
    let mut b = body(b"x", b"y", 1);
    b[1] = 21;
    assert!(decode_identity(&b).is_none());
    let mut b = body(b"x", b"y", 1);
    b[3] = 1;
    assert!(decode_identity(&b).is_none());
}

#[test]
fn nothing_but_printable_ascii_inside_a_length_or_zero_past_it() {
    for bad in [0x00u8, 0x1f, 0x7f, 0x80, 0xff] {
        let mut b = body(b"abc", b"1234", 1);
        b[5] = bad;
        assert!(decode_identity(&b).is_none(), "model byte {bad:#x}");
        let mut b = body(b"abc", b"1234", 1);
        b[46] = bad;
        assert!(decode_identity(&b).is_none(), "serial byte {bad:#x}");
    }
    let mut b = body(b"abc", b"1234", 1);
    b[4 + 3] = b'x';
    assert!(decode_identity(&b).is_none(), "model padding");
    let mut b = body(b"abc", b"1234", 1);
    b[63] = b'x';
    assert!(decode_identity(&b).is_none(), "serial padding");
}
