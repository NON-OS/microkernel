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

//! Mutation fuzzing, deterministic and seeded, run as an ordinary test.
//!
//! This is not coverage-guided: no libFuzzer is vendored in this tree. It
//! takes every vector, damages it the ways hostile bytes arrive (bit flips,
//! truncation, spliced noise, overwritten lengths), and asserts the decoder
//! returns without panicking, overflowing or producing past its bound. Debug
//! builds trap on arithmetic overflow, so this runs without --release.

#[path = "support/damage.rs"]
mod damage;
#[path = "support/rng.rs"]
mod rng;

use damage::mutate;
use nonos_zstd::{decompress, MAX_OUT};
use rng::Rng;

const ROUNDS: usize = 4000;
const SMALL: [&str; 6] = ["tiny", "text64k", "nocheck", "concat", "runs100k", "stream"];

#[test]
fn hostile_bytes_never_panic() {
    let mut r = Rng::new(0x5EED);
    let mut accepted = 0;
    for name in SMALL {
        let path = format!("{}/tests/vectors/{name}.zst", env!("CARGO_MANIFEST_DIR"));
        let clean = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        for _ in 0..ROUNDS {
            let mut v = clean.clone();
            mutate(&mut r, &mut v);
            if let Some(out) = decompress(&v) {
                assert!(out.len() <= MAX_OUT);
                accepted += 1;
            }
        }
    }
    // Most damage must be caught; the count is printed so a drift is visible.
    println!("accepted {accepted} of {} mutants", SMALL.len() * ROUNDS);
}

#[test]
fn pure_noise_never_panics() {
    let mut r = Rng::new(0xBAD);
    for _ in 0..ROUNDS {
        let mut v = 0xFD2F_B528u32.to_le_bytes().to_vec();
        let n = (r.next() % 256) as usize;
        v.extend((0..n).map(|_| (r.next() >> 56) as u8));
        let _ = decompress(&v);
    }
}
