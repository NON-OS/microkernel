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

//! A GPT header the kernel must not believe, as Linux's `is_gpt_valid`
//! refuses it: each field off by itself, its CRC made good again so only
//! the field is wrong; the CRC itself broken; a disk smaller than the table.

extern crate alloc;

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/files.rs"]
mod files;
#[path = "common/gpt_damage.rs"]
mod gpt_damage;
#[path = "common/kernel_gpt/mod.rs"]
#[allow(dead_code, unused_imports)]
mod kernel_gpt;
#[path = "common/mem_disk.rs"]
mod mem_disk;
#[path = "common/written.rs"]
mod written;

use gpt_damage::{put, seal, table, T};
use kernel_gpt::header;

#[test]
fn each_header_field_off_by_itself_is_refused() {
    let (h0, a) = table();
    assert!(header::parse(&h0, T).is_some(), "as written");
    let cases: [(usize, u64, usize); 10] = [
        (0, 0x5452_4150_2049_4646, 8), // signature "FFI PART"
        (12, 91, 4),                   // header size under 92
        (12, 513, 4),                  // header size past the sector
        (24, 2, 8),                    // my LBA not 1
        (48, T, 8),                    // last usable past the disk
        (40, T - 2, 8),                // first usable past last usable
        (72, 1, 8),                    // entries over the header
        (80, 0, 4),                    // no entries
        (80, 1025, 4),                 // more entries than are read
        (84, 256, 4),                  // entries not 128 bytes
    ];
    for (at, v, width) in cases {
        let mut h = h0.clone();
        put(&mut h, at, v, width);
        seal(&mut h, &a);
        assert!(header::parse(&h, T).is_none(), "field at {at} = {v:#x}");
    }
    let mut h = h0.clone();
    h[60] ^= 1;
    assert!(header::parse(&h, T).is_none(), "the header CRC");
    assert!(header::parse(&h0, 2000).is_none(), "a disk smaller than the table says");
}
