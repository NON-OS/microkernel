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

//! Every footer the loader's parser could meet, changed one byte at a time or
//! drawn at random, is read by the kernel exactly as the loader reads it.

use super::footer_tests::{loader, ours};
use crate::fixture::{signed_file, HAS_ZK_PROOF};

#[test]
fn every_footer_byte_changed_is_read_as_the_loader_reads_it() {
    let f = signed_file(&[0x7F; 2048], &[0xC4; 200], HAS_ZK_PROOF, 1);
    let at = f.len() - 64;
    for i in at..f.len() {
        for mask in [0x01u8, 0x02, 0x10, 0x80, 0xFF] {
            let mut g = f.clone();
            g[i] ^= mask;
            assert_eq!(ours(&g), loader(&g), "footer byte {} ^ {mask:#x}", i - at);
        }
    }
}

#[test]
fn random_regions_are_read_as_the_loader_reads_them() {
    let f = signed_file(&[0x7F; 1024], &[0xC4; 128], HAS_ZK_PROOF, 2);
    let (at, span) = (f.len() - 64, f.len() as u32 + 64);
    let (mut x, mut admitted) = (0x9E37_79B9_7F4A_7C15u64, 0);
    for _ in 0..200_000 {
        let mut g = f.clone();
        for _ in 0..2 {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            let field = at + 24 + 4 * (x % 6) as usize;
            g[field..field + 4].copy_from_slice(&((x >> 16) as u32 % span).to_le_bytes());
        }
        g[at + 10] = (x >> 56) as u8 & 3;
        assert_eq!(ours(&g), loader(&g), "footer {:02x?}", &g[at..]);
        admitted += usize::from(ours(&g).is_some());
    }
    assert!(admitted > 100, "the comparison reached accepted footers too: {admitted}");
}
