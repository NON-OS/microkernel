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

//! A table of entries larger than 128 bytes, which UEFI 2.10 5.3.2 allows
//! as 128 x 2^n (as recalled, unverified): believed, and each entry read
//! for the first 128 bytes, where every field the kernel reads lies.

use crate::gpt_damage::{put, seal, table, T};
use crate::kernel_gpt::Which;
use crate::kernel_gpt::{entries, header};

#[test]
fn entries_of_256_bytes_are_read_for_their_first_128() {
    let (h0, a) = table();
    let want = entries(&header::parse_as(&h0, T, Which::Primary).unwrap(), &a).unwrap();
    let mut wide = vec![0u8; 64 * 256];
    for (i, e) in a.chunks(128).take(64).enumerate() {
        // The second half repeats the first: read as 128-byte entries, every
        // partition would be named twice and overlap itself.
        wide[i * 256..i * 256 + 128].copy_from_slice(e);
        wide[i * 256 + 128..(i + 1) * 256].copy_from_slice(e);
    }
    let mut h = h0.clone();
    put(&mut h, 80, 64, 4);
    put(&mut h, 84, 256, 4);
    seal(&mut h, &wide);
    let hd = header::parse_as(&h, T, Which::Primary)
        .expect("64 entries of 256 bytes fill the same 32 sectors");
    assert_eq!(entries(&hd, &wide).unwrap(), want);
}
