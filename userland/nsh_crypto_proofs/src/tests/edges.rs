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

//! What the opener and the sealer do around their edges: a frame past the
//! opener's byte limit is refused and ends it; a length mismatch ends it; a
//! buffer of the caller's that is too small leaves it as it was; a seal
//! that fails is not counted; a frame with no type byte is refused by both.

use super::sealer::{frame, open};
use nsh_crypto::{CryptoError, Opener, Sealer, Secret, MAX_SEALED_BYTES};

pub fn key() -> Secret<32> {
    Secret::new([3; 32])
}

#[test]
fn a_frame_past_the_openers_byte_limit_is_refused_and_ends_it() {
    let small = frame(&mut Sealer::new(key()), b"\x31");
    let big = frame(&mut Sealer::new(key()), &[0x31; 10]);
    let mut o = Opener::starting_at(key(), 0, MAX_SEALED_BYTES - 4);
    assert_eq!(open(&mut o, &big), Err(CryptoError::Limit));
    assert_eq!(open(&mut o, &small), Err(CryptoError::Open), "a frame that fits is refused too");
}

#[test]
fn the_opener_counts_the_bytes_it_opens() {
    let mut s = Sealer::new(key());
    let (a, b) = (frame(&mut s, b"\x31ab"), frame(&mut s, b"\x31c"));
    let mut o = Opener::starting_at(key(), 0, MAX_SEALED_BYTES - 4);
    assert_eq!(open(&mut o, &a), Ok(b"\x31ab".to_vec()));
    assert_eq!(open(&mut o, &b), Err(CryptoError::Limit), "3 + 2 bytes are past the last 4");
}

#[test]
fn a_length_mismatch_ends_the_opener() {
    let mut s = Sealer::new(key());
    let f = frame(&mut s, b"\x30a");
    let mut o = Opener::new(key());
    assert_eq!(o.open(99u32.to_le_bytes(), &f.1, &mut [0u8; 32]), Err(CryptoError::Open));
    assert_eq!(open(&mut o, &f), Err(CryptoError::Open));
}

#[test]
fn a_short_buffer_is_the_callers_error_and_ends_nothing() {
    let mut s = Sealer::new(key());
    let f = frame(&mut s, b"\x30abcdef");
    let mut o = Opener::new(key());
    assert_eq!(o.open(f.0, &f.1, &mut [0u8; 3]), Err(CryptoError::Room));
    assert_eq!(open(&mut o, &f), Ok(b"\x30abcdef".to_vec()));
}
