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

//! Sealed frames changed, sent the wrong way, or given the wrong length are
//! refused.

use super::sealer::{frame, open, pair};
use nsh_crypto::CryptoError;

#[test]
fn a_changed_byte_anywhere_is_refused() {
    let (mut s, _) = pair(3);
    let f = frame(&mut s, b"\x31output");
    for i in 0..f.1.len() {
        let mut bad = f.clone();
        bad.1[i] ^= 0x01;
        assert_eq!(open(&mut pair(3).1, &bad), Err(CryptoError::Open), "byte {i}");
    }
}

#[test]
fn a_frame_sealed_one_way_does_not_open_the_other_way() {
    let ss = nsh_crypto::Secret::new([4; 32]);
    let (keys, _) = nsh_crypto::handshake_keys(ss, &[5; 64]);
    let mut c2s = nsh_crypto::Sealer::new(nsh_crypto::Secret::new(*keys.c2s.expose()));
    let mut s2c_opener = nsh_crypto::Opener::new(nsh_crypto::Secret::new(*keys.s2c.expose()));
    let f = frame(&mut c2s, b"\x30x");
    assert_eq!(open(&mut s2c_opener, &f), Err(CryptoError::Open));
}

#[test]
fn a_length_that_disagrees_with_the_bytes_is_refused() {
    let (mut s, mut o) = pair(6);
    let mut out = [0u8; 32];
    assert_eq!(s.seal(21u32.to_le_bytes(), b"\x30abc", &mut out), Err(CryptoError::Room));
    assert_eq!(o.open(16u32.to_le_bytes(), &[0; 16], &mut out), Err(CryptoError::Open));
}

#[test]
fn a_frame_whose_length_field_disagrees_is_refused_even_with_a_good_tag() {
    // A peer's own sealer cannot make this; one built by hand can, and the
    // tag is good because the length bytes it covers are the ones sent.
    let key = [9u8; 32];
    let len = 40u32.to_le_bytes();
    let mut sealed = [0u8; 20];
    nonos_seal::seal(&key, &[0; 12], &len, b"\x30abc", &mut sealed).unwrap();
    let mut o = nsh_crypto::Opener::new(nsh_crypto::Secret::new(key));
    assert_eq!(o.open(len, &sealed, &mut [0u8; 20]), Err(CryptoError::Open));
}
