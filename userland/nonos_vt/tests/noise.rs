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

//! Pure noise: random bytes with no structure, with the host resizing,
//! scrolling and copying between them, which must never panic or leave the
//! screen unsound.

#[path = "support/check.rs"]
mod check;
#[path = "support/rng.rs"]
mod rng;

use check::{check, poke};
use nonos_vt::Term;
use rng::Rng;

#[test]
fn pure_noise_never_panics() {
    let mut r = Rng::new(0xBAD);
    let mut t = Term::new(80, 24, 1000);
    for round in 0..20_000 {
        let n = (r.next() % 64) as usize;
        let bytes: Vec<u8> = (0..n).map(|_| (r.next() >> 56) as u8).collect();
        t.feed(&bytes);
        poke(&mut t, &mut r, round);
        check(&mut t);
    }
}
