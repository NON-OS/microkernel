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

//! How an HkdfLabel is framed, and what will not fit in one.

use crate::hkdf_label::hkdf_label;

#[test]
fn the_length_byte_counts_the_tls13_prefix() {
    for label in [b"key".as_slice(), b"iv", b"derived", b"c hs traffic", b"finished"] {
        let got = hkdf_label(32, label, &[]).expect("fits");
        assert_eq!(got[2] as usize, label.len() + 6, "label length includes the prefix");
        assert_eq!(&got[3..9], b"tls13 ", "and the prefix is what it counts");
    }
}
#[test]
fn the_requested_length_is_big_endian() {
    assert_eq!(&hkdf_label(1, b"x", &[]).expect("fits")[0..2], &[0x00, 0x01]);
    assert_eq!(&hkdf_label(256, b"x", &[]).expect("fits")[0..2], &[0x01, 0x00]);
    assert_eq!(&hkdf_label(65535, b"x", &[]).expect("fits")[0..2], &[0xFF, 0xFF]);
}
#[test]
fn a_field_that_will_not_fit_is_refused_rather_than_truncated() {
    assert!(hkdf_label(32, &[b'x'; 250], &[]).is_none(), "label plus prefix overruns a byte");
    assert!(hkdf_label(32, b"key", &[0u8; 256]).is_none(), "context overruns a byte");
    assert!(hkdf_label(65536, b"key", &[]).is_none(), "length overruns two bytes");

    // And the largest values that do fit are still built.
    assert!(hkdf_label(65535, &[b'x'; 249], &[0u8; 255]).is_some());
}
