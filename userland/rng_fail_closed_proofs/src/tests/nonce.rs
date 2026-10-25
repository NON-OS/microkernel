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

//! A token's 128-bit nonce is refused before seeding, never zeros. The
//! 64-bit nonce, which only has to be unique, falls back to a count.

use crate::nonce::{current_nonce_counter, default_nonce, secure_nonce_128};
use crate::stand_in::{sources, SEEDED_TAG};

#[test]
fn the_128_bit_nonce_is_refused_before_seeding() {
    sources(false, true, true);
    assert_eq!(secure_nonce_128(), Err("no randomness"));
}

#[test]
fn the_128_bit_nonce_comes_from_the_seeded_rng() {
    sources(true, true, false);
    let n = secure_nonce_128().expect("drawn");
    assert_eq!(u64::from_le_bytes(n[..8].try_into().unwrap()) >> 56, SEEDED_TAG);
    assert_eq!(u64::from_le_bytes(n[8..].try_into().unwrap()) >> 56, SEEDED_TAG);
}

// The only test that moves the shared count.
#[test]
fn the_64_bit_nonce_counts_when_it_cannot_draw() {
    sources(true, true, false);
    let before = current_nonce_counter();
    assert_eq!(default_nonce() >> 56, SEEDED_TAG);
    assert_eq!(current_nonce_counter(), before, "a drawn nonce leaves the count");
    sources(false, true, true);
    let (a, b) = (default_nonce(), default_nonce());
    assert_eq!((a, b), (before, before + 1));
    assert_eq!(current_nonce_counter(), before + 2);
}
