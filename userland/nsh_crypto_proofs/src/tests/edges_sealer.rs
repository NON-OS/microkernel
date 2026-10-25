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

//! A seal that fails is not counted, and a frame with no type byte is refused
//! by both sides.

use super::edges::key;
use super::sealer::{frame, open};
use nsh_crypto::{CryptoError, Opener, Sealer};

#[test]
fn a_seal_that_fails_is_not_counted() {
    let mut s = Sealer::new(key());
    assert_eq!(s.seal(20u32.to_le_bytes(), b"\x30abc", &mut [0u8; 5]), Err(CryptoError::Room));
    let f = frame(&mut s, b"\x30abc");
    assert_eq!(open(&mut Opener::new(key()), &f), Ok(b"\x30abc".to_vec()), "still sequence 0");
}

#[test]
fn a_frame_with_no_type_byte_is_refused_by_both_sides() {
    let mut out = [0u8; 16];
    assert_eq!(Sealer::new(key()).seal(16u32.to_le_bytes(), b"", &mut out), Err(CryptoError::Room));
    // An empty frame with a good tag, as only a hand-built sealer makes one.
    let mut sealed = [0u8; 16];
    nonos_seal::seal(&[3; 32], &[0; 12], &16u32.to_le_bytes(), b"", &mut sealed).unwrap();
    let r = Opener::new(key()).open(16u32.to_le_bytes(), &sealed, &mut [0u8; 16]);
    assert_eq!(r, Err(CryptoError::Open));
}
