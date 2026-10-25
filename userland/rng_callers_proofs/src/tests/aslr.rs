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

//! The ASLR slide (security/policy/advanced/cfi.rs) comes only from the
//! seeded RNG; with none, no address is given.

use crate::security::policy::advanced::cfi::ASLR;
use crate::stand_in::rng;

const BASE: u64 = 0x40_0000;

#[test]
fn no_slide_is_made_with_no_randomness() {
    rng::refuse();
    assert_eq!(ASLR::new(28).randomize(BASE), None);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn each_slide_is_the_drawn_word_masked() {
    let aslr = ASLR::new(28);
    rng::fail_from(None);
    let (a, b) = (aslr.randomize(BASE).expect("slid"), aslr.randomize(BASE).expect("slid"));
    let word = |g: &Vec<u8>| u64::from_le_bytes(g[..].try_into().expect("8 bytes"));
    let given = rng::given();
    for (got, drawn) in [(a, &given[0]), (b, &given[1])] {
        assert_eq!(got, BASE + ((word(drawn) & ((1 << 28) - 1)) << 12));
    }
    assert_ne!(a, b);
}
