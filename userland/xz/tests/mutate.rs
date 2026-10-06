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

//! Seeded mutation fuzzing of the xz container and LZMA2, not
//! coverage-guided (no libFuzzer is vendored). Every damaged input must
//! return, in a debug build, without a panic, an overflow or runaway output.

#[path = "../../zstd/tests/support/damage.rs"]
mod damage;
#[path = "../../zstd/tests/support/rng.rs"]
mod rng;

use damage::mutate;
use nonos_xz::{decompress, MAX_OUT};
use rng::Rng;

const ROUNDS: usize = 2000;
const SMALL: [&str; 6] = ["tiny", "text64k", "none", "concat", "lc4pb4", "dict4k"];

#[test]
fn hostile_bytes_never_panic() {
    let mut r = Rng::new(0x5EED_0F_A2);
    let mut accepted = 0;
    for name in SMALL {
        let path = format!("{}/tests/vectors/{name}.xz", env!("CARGO_MANIFEST_DIR"));
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
    println!("accepted {accepted} of {} mutants", SMALL.len() * ROUNDS);
}
