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

//! Every encoding over a spread of lengths and all four levels, held to the
//! digest the code gave before its clippy fixes (d26d1b15's parent), so an
//! edit that changes any module of any sample fails here, not on a phone.

use nonos_qr::{encode, Ecc};

/// The data for one sample: a byte pattern of `len` bytes, the same every run.
fn data(len: usize) -> Vec<u8> {
    let mut x: u32 = 0x9E37_79B9 ^ len as u32;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            x as u8
        })
        .collect()
}

/// FNV-1a over every sample's size and modules, a marker where encode refuses.
fn digest() -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |b: u8| {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    };
    for len in (0..3000).step_by(23) {
        for ecc in [Ecc::Low, Ecc::Medium, Ecc::Quartile, Ecc::High] {
            match encode(&data(len), ecc) {
                Some(q) => {
                    (q.size as u32).to_le_bytes().into_iter().for_each(&mut eat);
                    q.modules.iter().for_each(|&m| eat(m as u8));
                }
                None => eat(0xFF),
            }
        }
    }
    h
}

/// What d26d1b15's parent gave for these samples: 131 lengths from 0 to 2,990
/// at each level, the longest past version 40's capacity and refused.
const PINNED: u64 = 0x8399_84ad_24d7_6785;

#[test]
fn every_sampled_encoding_is_the_one_pinned() {
    assert_eq!(digest(), PINNED);
}
