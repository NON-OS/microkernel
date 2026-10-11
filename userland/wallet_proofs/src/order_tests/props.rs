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

//! The claims `kani_order.rs` proves, checked here on a million words built
//! to hit every byte position: orderOf, state and extra, and a balance word
//! are read only when their high bytes are zero, and then exactly.

use crate::landed::balance_of;
use crate::order::order_of::order_of;
use crate::order::refund_call::state_and_extra;

fn words(seed: u64) -> [[u8; 32]; 5] {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut w = [[0u8; 32]; 5];
    for (i, b) in w.iter_mut().flatten().enumerate() {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        // The low bytes always, a high byte rarely: both paths are taken.
        let low = if i < 32 { 31 } else { 24 };
        *b = if i % 32 >= low || (x >> 8) % 97 == 0 { x as u8 } else { 0 };
    }
    w
}

fn zero_to(w: &[u8; 32], n: usize) -> bool {
    w[..n].iter().all(|b| *b == 0)
}

#[test]
fn the_escrow_words_are_read_only_when_they_fit() {
    let mut read = 0;
    for seed in 0..200_000u64 {
        let w = words(seed);
        let small = zero_to(&w[0], 31) && zero_to(&w[4], 24);
        match order_of(&w) {
            Some((state, at)) => {
                assert!(small, "seed {seed}");
                assert_eq!(state, w[0][31]);
                assert_eq!(at, u64::from_be_bytes(w[4][24..].try_into().unwrap()));
                read += 1;
            }
            None => assert!(!small, "seed {seed}"),
        }
        let fits = zero_to(&w[0], 31) && zero_to(&w[2], 16);
        assert_eq!(state_and_extra(&w).is_some(), fits, "seed {seed}");
        assert_eq!(balance_of(&w[1..2]).is_some(), zero_to(&w[1], 16), "seed {seed}");
    }
    assert!(read > 0, "the read path was taken");
}
