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

//! The secure byte calls fail before the RNG is seeded, even while the
//! hardware answers: they never fall back.

use crate::crypto::error::CryptoError;
use crate::crypto::random_api::{get_bytes_secure, get_vec_secure};
use crate::crypto::util::misc::{secure_random_u32, secure_random_u64};
use crate::crypto::util::rng::RngError;
use crate::stand_in::{sources, SEEDED_TAG};

#[test]
fn get_bytes_secure_refuses_before_seeding() {
    sources(false, true, true);
    let mut buf = [0xaau8; 24];
    assert_eq!(get_bytes_secure(&mut buf), Err(CryptoError::InsufficientEntropy));
    assert_eq!(buf, [0xaa; 24], "no fallback bytes were written");
    assert_eq!(get_vec_secure(40), Err(CryptoError::InsufficientEntropy));
}

#[test]
fn get_bytes_secure_draws_from_the_seeded_rng() {
    sources(true, true, false);
    let mut buf = [0u8; 16];
    assert_eq!(get_bytes_secure(&mut buf), Ok(()));
    for word in buf.chunks(8) {
        assert_eq!(u64::from_le_bytes(word.try_into().unwrap()) >> 56, SEEDED_TAG);
    }
    let v = get_vec_secure(40).expect("drawn");
    assert!(v.len() == 40 && v.chunks(8).all(|w| w[7] == SEEDED_TAG as u8));
}

#[test]
fn misc_refuses_before_seeding() {
    sources(false, true, true);
    assert_eq!(secure_random_u64(), Err(RngError::NotInitialized));
    assert_eq!(secure_random_u32(), Err(RngError::NotInitialized));
}

#[test]
fn misc_draws_from_the_seeded_rng() {
    sources(true, false, false);
    assert_eq!(secure_random_u64().map(|v| v >> 56), Ok(SEEDED_TAG));
    assert_eq!(secure_random_u32().map(|v| v >> 24), Ok(SEEDED_TAG as u32));
}
