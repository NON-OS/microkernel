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

//! The streaming SHA-256 agrees with the one-shot at every length and every
//! split, and both give the FIPS 180-4 answers.

use nonos_hash::{sha256, Sha256};

#[test]
fn fips_answers() {
    let abc = sha256(b"abc");
    assert_eq!(abc[..4], [0xba, 0x78, 0x16, 0xbf]);
    assert_eq!(abc[28..], [0xf2, 0x00, 0x15, 0xad]);
    let mut s = Sha256::new();
    s.update(b"ab");
    s.update(b"c");
    assert_eq!(s.finalize(), abc);
}

#[test]
fn every_split_matches_the_one_shot() {
    let data: Vec<u8> = (0..300u32).map(|i| (i * 31 + 7) as u8).collect();
    for len in 0..data.len() {
        let want = sha256(&data[..len]);
        for cut in 0..=len {
            let mut s = Sha256::new();
            s.update(&data[..cut]);
            s.update(&data[cut..len]);
            assert_eq!(s.finalize(), want, "len {len} cut {cut}");
        }
    }
}
