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

//! A GPT header the kernel must not believe, each check UEFI 2.10 5.3.2's
//! own (as recalled, unverified): each field off by itself, its CRC made
//! good again so only the field is wrong, and set so no other check
//! refuses it too; the CRC itself broken; a disk smaller than the table.

use crate::gpt_damage::{put, seal, table, T};
use crate::kernel_gpt::{header, Which};
type Field = (usize, u64, usize); // offset, value, width in bytes

#[test]
fn each_header_field_off_by_itself_is_refused() {
    let (h0, a) = table();
    assert!(header::parse_as(&h0, T, Which::Primary).is_some(), "as written");
    // (field, value, width, and a field set first so this check alone fails)
    let cases: [(usize, u64, usize, Option<Field>, &str); 13] = [
        (0, 0x5452_4150_2049_4646, 8, None, "signature not EFI PART"),
        (12, 91, 4, None, "HeaderSize under its 92 bytes of fields"),
        (12, 513, 4, None, "HeaderSize past the 512-byte block"),
        (24, 2, 8, None, "MyLBA not the LBA read"),
        (48, T, 8, None, "LastUsableLBA past the disk"),
        (40, T - 2, 8, None, "FirstUsableLBA past LastUsableLBA"),
        (72, 1, 8, None, "the array over the header"),
        (72, 3, 8, None, "the array reaching into the usable range"),
        (80, 0, 4, None, "no entries"),
        (80, 1025, 4, Some((40, 300, 8)), "past the 128 KiB read"),
        (84, 192, 4, Some((80, 64, 4)), "an entry size not 128 x 2^n"),
        (84, 64, 4, None, "an entry size under 128"),
        (84, 256, 4, None, "128 entries of 256 bytes reach into the usable range"),
    ];
    for (at, v, width, first, why) in cases {
        let mut h = h0.clone();
        if let Some((at, v, width)) = first {
            put(&mut h, at, v, width);
        }
        put(&mut h, at, v, width);
        seal(&mut h, &a);
        assert!(header::parse_as(&h, T, Which::Primary).is_none(), "{why}");
    }
    let mut h = h0.clone();
    h[60] ^= 1;
    assert!(header::parse_as(&h, T, Which::Primary).is_none(), "the header CRC");
    assert!(header::parse_as(&h0, 2000, Which::Primary).is_none(), "a disk smaller than its table");
}
