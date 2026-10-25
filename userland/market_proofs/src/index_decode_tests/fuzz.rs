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

//! OP_LOAD_INDEX hands its whole body to decode_index before any signature
//! is checked, so the decode reads bytes nobody has vouched for yet. Seeded
//! from blobs the abi's own encoder writes, then damaged where the length
//! and count fields live, every input either decodes within the abi's caps,
//! with the signed bytes a prefix of the blob, or is refused; and a count past
//! a cap is refused before anything is reserved for it.

use nonos_marketplace_abi::{MAX_ENTRIES, MAX_RELEASES};

use super::check::check;
use super::seeds::{seeds, xorshift, FUZZ_ROUNDS};

#[test]
fn damaged_blobs_decode_within_the_caps_or_not_at_all() {
    let seeds = seeds();
    let mut s = 0x4E4D_4B54_0000_0001u64;
    let mut decoded = 0usize;
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let mut b = seeds[(r % 3) as usize].clone();
        for _ in 0..1 + (r >> 8) % 3 {
            let at = (xorshift(&mut s) % b.len() as u64) as usize;
            match xorshift(&mut s) % 6 {
                // A length or count field: small, at a cap, past it, or huge.
                0 | 1 => {
                    let v: u32 = match xorshift(&mut s) % 6 {
                        0 => (xorshift(&mut s) % 8) as u32,
                        1 => MAX_ENTRIES,
                        2 => MAX_ENTRIES + 1,
                        3 => MAX_RELEASES + (xorshift(&mut s) % 2) as u32,
                        4 => u32::MAX - (xorshift(&mut s) % 4) as u32,
                        _ => xorshift(&mut s) as u32,
                    };
                    let end = (at + 4).min(b.len());
                    b[at..end].copy_from_slice(&v.to_le_bytes()[..end - at]);
                }
                2 => b[at] ^= 1 << (xorshift(&mut s) % 8),
                3 => b.truncate(at),
                4 => b.insert(at, xorshift(&mut s) as u8),
                _ => {
                    b.remove(at);
                }
            }
            if b.is_empty() {
                break;
            }
        }
        if check(&b) {
            decoded += 1;
        }
    }
    assert!(decoded > FUZZ_ROUNDS / 50, "the generator reaches whole decodes: {decoded}");
}
