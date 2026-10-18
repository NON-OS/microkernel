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

//! Both entry point forms, their sums and their fields.

use crate::entry::{parse_entry, Entry};

fn fix(b: &mut [u8], at: usize, over: core::ops::Range<usize>) {
    b[at] = 0;
    b[at] = 0u8.wrapping_sub(b[over].iter().fold(0u8, |s, x| s.wrapping_add(*x)));
}

#[test]
fn sm3_and_sm() {
    let mut b = vec![0u8; 24];
    b[..5].copy_from_slice(b"_SM3_");
    (b[6], b[7], b[8]) = (24, 3, 6);
    b[12..16].copy_from_slice(&0x400u32.to_le_bytes());
    b[16..24].copy_from_slice(&0x7FFE_0000u64.to_le_bytes());
    fix(&mut b, 5, 0..24);
    let want = Entry { major: 3, minor: 6, table: 0x7FFE_0000, len: 0x400, count: None };
    assert_eq!(parse_entry(&b), Some(want));
    let mut c = vec![0u8; 0x1F];
    c[..4].copy_from_slice(b"_SM_");
    (c[5], c[6], c[7]) = (0x1F, 2, 8);
    c[0x10..0x15].copy_from_slice(b"_DMI_");
    c[0x16..0x18].copy_from_slice(&0x1A0u16.to_le_bytes());
    c[0x18..0x1C].copy_from_slice(&0xF0000u32.to_le_bytes());
    c[0x1C..0x1E].copy_from_slice(&9u16.to_le_bytes());
    fix(&mut c, 0x15, 0x10..0x1F);
    fix(&mut c, 4, 0..0x1F);
    let want = Entry { major: 2, minor: 8, table: 0xF0000, len: 0x1A0, count: Some(9) };
    assert_eq!(parse_entry(&c), Some(want));
    c[0x1C] ^= 1;
    assert_eq!(parse_entry(&c), None, "both sums must hold");
    assert_eq!(parse_entry(b"_SM3_"), None, "short");
}
