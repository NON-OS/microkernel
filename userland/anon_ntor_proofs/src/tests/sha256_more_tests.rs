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
//! Sha256, continued.

use crate::sha256::digest;
use crate::sha256::types::Sha256;
use alloc::vec;

fn hex(bytes: &[u8]) -> alloc::string::String {
    use core::fmt::Write;
    let mut out = alloc::string::String::new();
    for b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

#[test]
fn a_consensus_sized_input_hashes_in_one_piece_and_in_many() {
    let big = vec![0x5au8; 1_791_340];
    let whole = digest(&big);

    let mut split = Sha256::new();
    for chunk in big.chunks(4096) {
        split.update(chunk);
    }
    assert_eq!(split.finish(), whole, "a split input must hash as the whole");

    // And across awkward boundaries rather than exact block multiples.
    let mut ragged = Sha256::new();
    let mut at = 0;
    for step in [1usize, 63, 64, 65, 127, 4095].iter().cycle() {
        if at >= big.len() {
            break;
        }
        let end = (at + step).min(big.len());
        ragged.update(&big[at..end]);
        at = end;
    }
    assert_eq!(ragged.finish(), whole, "block boundaries must not change the answer");
}
#[test]
fn the_padding_boundary() {
    for len in [54usize, 55, 56, 57, 63, 64, 65] {
        let input = vec![b'x'; len];
        let mut streamed = Sha256::new();
        streamed.update(&input);
        assert_eq!(streamed.finish(), digest(&input), "{len} bytes");
    }
    assert_eq!(
        hex(&digest(&vec![b'x'; 56])[..8]),
        hex(&digest(&vec![b'x'; 56])[..8]),
        "stable across calls"
    );
}
