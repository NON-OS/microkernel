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
//! Not coverage-guided: no libFuzzer is vendored in this tree. It builds
//! byte streams from the pieces hostile output is made of (control
//! sequences with random parameters and finals, strings, broken UTF-8, wide
//! and combining characters, raw noise), interleaves resizes, scrolling,
//! copying and searching, and after every step checks what must always
//! hold: the cursor on screen, every screen row as wide as the screen, the
//! reply queue under its ceiling. Debug builds trap on arithmetic overflow,
//! so an overflow anywhere fails the test.

use nonos_vt::Term;

#[path = "support/check.rs"]
mod check;
#[path = "support/pieces.rs"]
mod pieces;
#[path = "support/rng.rs"]
mod rng;

use check::{check, poke};
use pieces::piece;
use rng::Rng;

#[test]
fn hostile_output_keeps_the_screen_sound() {
    // A different seed explores different streams: NONOS_VT_SEED=n.
    let seed = std::env::var("NONOS_VT_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(0x7E57);
    let mut r = Rng::new(seed);
    for round in 0..400 {
        let mut t = Term::new(1 + (r.next() % 120) as usize, 1 + (r.next() % 50) as usize, 200);
        for _ in 0..200 {
            let mut bytes = Vec::new();
            for _ in 0..(r.next() % 8) {
                piece(&mut r, &mut bytes);
            }
            // Feed in pieces, so sequences split across reads too.
            let cut = (r.next() as usize) % (bytes.len() + 1);
            t.feed(&bytes[..cut]);
            t.feed(&bytes[cut..]);
            poke(&mut t, &mut r, round);
            check(&mut t);
        }
    }
}
