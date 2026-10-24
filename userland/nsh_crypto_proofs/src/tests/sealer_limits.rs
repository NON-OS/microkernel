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

//! A direction stops at 2^32 frames or 2^32 sealed bytes, on both the
//! sealing and the opening side; the limit is reached exactly, never
//! passed, and a frame refused for the limit is not counted.

use super::sealer::{frame, open};
use nsh_crypto::{CryptoError, Opener, Sealer, Secret, MAX_SEALED_BYTES, MAX_SEALED_FRAMES};

fn key() -> Secret<32> {
    Secret::new([8; 32])
}

#[test]
fn the_last_frame_is_sealed_and_opened_and_the_next_is_refused() {
    assert_eq!((MAX_SEALED_FRAMES, MAX_SEALED_BYTES), (1 << 32, 1 << 32));
    let mut s = Sealer::starting_at(key(), MAX_SEALED_FRAMES - 1, 0);
    let mut o = Opener::starting_at(key(), MAX_SEALED_FRAMES - 1, 0);
    let last = frame(&mut s, b"\x52");
    assert_eq!(open(&mut o, &last), Ok(b"\x52".to_vec()));
    let mut out = [0u8; 17];
    assert_eq!(s.seal(17u32.to_le_bytes(), b"\x52", &mut out), Err(CryptoError::Limit));
    assert_eq!(o.open(17u32.to_le_bytes(), &out, &mut [0u8; 17]), Err(CryptoError::Limit));
}

#[test]
fn the_last_byte_is_sealed_and_one_more_is_refused() {
    let mut s = Sealer::starting_at(key(), 0, MAX_SEALED_BYTES - 4);
    let mut o = Opener::starting_at(key(), 0, MAX_SEALED_BYTES - 4);
    let f = frame(&mut s, b"\x30abc");
    assert_eq!(open(&mut o, &f), Ok(b"\x30abc".to_vec()));
    let mut out = [0u8; 17];
    assert_eq!(s.seal(17u32.to_le_bytes(), b"\x30", &mut out), Err(CryptoError::Limit));
}

#[test]
fn a_frame_refused_for_the_limit_is_not_counted() {
    let mut s = Sealer::starting_at(key(), 0, MAX_SEALED_BYTES - 2);
    let mut out = [0u8; 19];
    assert_eq!(s.seal(19u32.to_le_bytes(), b"\x30ab", &mut out), Err(CryptoError::Limit));
    let mut o = Opener::starting_at(key(), 0, MAX_SEALED_BYTES - 2);
    let f = frame(&mut s, b"\x30a");
    assert_eq!(open(&mut o, &f), Ok(b"\x30a".to_vec()), "still at sequence 0");
}
