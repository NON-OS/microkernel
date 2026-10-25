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

//! The core API's random calls (crypto/core/api/random/api.rs): each draws
//! only from the seeded RNG, refuses with `InsufficientEntropy` before it is
//! seeded, and never falls back.

use crate::crypto::CryptoError;
use crate::crypto::{fill_random, secure_random_u32, secure_random_u64, secure_random_u8};
use crate::stand_in::rng;

const NO: CryptoError = CryptoError::InsufficientEntropy;

#[test]
fn each_call_refuses_with_no_randomness() {
    rng::refuse();
    let mut buf = [0x77u8; 24];
    assert_eq!(fill_random(&mut buf), Err(NO));
    assert_eq!(buf, [0x77; 24]);
    assert_eq!(secure_random_u64(), Err(NO));
    assert_eq!(secure_random_u32(), Err(NO));
    assert_eq!(secure_random_u8(), Err(NO));
    assert_eq!((rng::draws(), rng::weak()), (4, 0));
}

#[test]
fn each_call_gives_the_drawn_bytes() {
    rng::fail_from(None);
    let mut buf = [0u8; 24];
    fill_random(&mut buf).expect("filled");
    let (w64, w32, w8) = (secure_random_u64(), secure_random_u32(), secure_random_u8());
    let g = rng::given();
    let word = |i: usize| u64::from_le_bytes(g[i][..].try_into().expect("8 bytes"));
    assert_eq!(g[0], buf);
    assert_eq!(w64, Ok(word(1)));
    assert_eq!(w32, Ok(word(2) as u32));
    assert_eq!(w8, Ok(word(3) as u8));
    assert_ne!(word(1), word(2));
}
