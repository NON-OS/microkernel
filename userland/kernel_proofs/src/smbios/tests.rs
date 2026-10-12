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

use super::smbios_entry::{entry_len, parse_entry, Entry, TABLE_MAX};

fn fix(b: &mut [u8], at: usize, over: core::ops::Range<usize>) {
    b[at] = 0;
    let s = b[over].iter().fold(0u8, |s, x| s.wrapping_add(*x));
    b[at] = 0u8.wrapping_sub(s);
}

fn sm3(table: u64, max: u32) -> Vec<u8> {
    let mut b = vec![0u8; 24];
    b[..5].copy_from_slice(b"_SM3_");
    (b[6], b[7], b[8]) = (24, 3, 6);
    b[12..16].copy_from_slice(&max.to_le_bytes());
    b[16..24].copy_from_slice(&table.to_le_bytes());
    fix(&mut b, 5, 0..24);
    b
}

fn sm2(table: u32, len: u16) -> Vec<u8> {
    let mut b = vec![0u8; 0x1F];
    b[..4].copy_from_slice(b"_SM_");
    (b[5], b[6], b[7]) = (0x1F, 2, 8);
    b[0x10..0x15].copy_from_slice(b"_DMI_");
    b[0x16..0x18].copy_from_slice(&len.to_le_bytes());
    b[0x18..0x1C].copy_from_slice(&table.to_le_bytes());
    fix(&mut b, 0x15, 0x10..0x1F);
    fix(&mut b, 4, 0..0x1F);
    b
}

#[test]
fn both_forms_read_and_sizes_bounded() {
    let e = Entry { table: 0x7FE0_0000, len: 0x200, major: 3, minor: 6 };
    assert_eq!(parse_entry(&sm3(0x7FE0_0000, 0x200)), Some(e));
    assert_eq!(entry_len(&sm3(0, 1)), Some(24));
    let e2 = Entry { table: 0xF0000, len: 0x1A0, major: 2, minor: 8 };
    assert_eq!(parse_entry(&sm2(0xF0000, 0x1A0)), Some(e2));
    assert_eq!(entry_len(&sm2(0, 1)), Some(0x1F));
    assert_eq!(parse_entry(&sm3(0x1000, TABLE_MAX + 1)), None, "too big to believe");
    assert_eq!(parse_entry(&sm3(0x1000, 0)), None);
}

#[test]
fn checksums_and_anchors_are_checked() {
    let mut b = sm3(0x1000, 0x100);
    b[20] ^= 1;
    assert_eq!(parse_entry(&b), None);
    let mut c = sm2(0x1000, 0x100);
    c[0x18] ^= 1;
    assert_eq!(parse_entry(&c), None, "the _DMI_ part's own sum");
    assert_eq!(parse_entry(b"_XX3_"), None);
    assert_eq!(entry_len(b"_SM3_\0\x40"), None, "longer than any entry point");
    assert_eq!(parse_entry(&sm3(0x1000, 0x100)[..10]), None, "short");
}
