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

//! A pin holds one file exactly: one bit, one octet short or one octet
//! more, and it holds nothing; and its digits are read as written.

use super::fixture::pin;
use super::rtl::pin::hex32;

#[test]
fn a_pin_holds_only_its_own_octets() {
    let bytes = b"a patch, every octet of it".to_vec();
    let p = pin("x_fw.bin", &bytes);
    assert!(p.holds(&bytes));
    let mut flipped = bytes.clone();
    flipped[7] ^= 0x01;
    assert!(!p.holds(&flipped), "one bit changed");
    assert!(!p.holds(&bytes[..bytes.len() - 1]), "one octet short");
    let mut longer = bytes.clone();
    longer.push(0);
    assert!(!p.holds(&longer), "one octet more");
}

#[test]
fn hex_digits_read_high_nibble_first() {
    let d = hex32("00ff102a00000000000000000000000000000000000000000000000000000001");
    assert_eq!(d[..4], [0x00, 0xff, 0x10, 0x2a]);
    assert_eq!(d[31], 0x01);
}

#[test]
#[should_panic]
fn an_upper_case_digit_is_refused() {
    hex32("00FF102a00000000000000000000000000000000000000000000000000000001");
}
