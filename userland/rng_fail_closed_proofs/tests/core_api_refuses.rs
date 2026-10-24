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

//! The core API's random calls draw from the seeded RNG only. Before seeding
//! they refuse, whatever the hardware answers, and never reach the fallback,
//! so neither boot line is said; once seeded, they answer from the RNG.

#![allow(dead_code)]

include!("../src/mount.rs");

use crypto::core_random::{fill_random, secure_random_u32, secure_random_u64, secure_random_u8};
use crypto::error::CryptoError;

#[test]
fn before_seeding_each_refuses_and_says_nothing() {
    stand_in::sources(false, true, true);
    let mut buf = [0xaa; 16];
    assert_eq!(fill_random(&mut buf), Err(CryptoError::InsufficientEntropy));
    assert_eq!(buf, [0xaa; 16], "nothing was written");
    assert_eq!(secure_random_u64(), Err(CryptoError::InsufficientEntropy));
    assert_eq!(secure_random_u32(), Err(CryptoError::InsufficientEntropy));
    assert_eq!(secure_random_u8(), Err(CryptoError::InsufficientEntropy));
    assert!(stand_in::SERIAL.lock().unwrap().is_empty(), "no fallback was reached");
    stand_in::sources(true, false, false);
    assert_eq!(secure_random_u64().map(|v| v >> 56), Ok(stand_in::SEEDED_TAG));
    assert!(fill_random(&mut buf).is_ok() && buf[7] == stand_in::SEEDED_TAG as u8);
}
