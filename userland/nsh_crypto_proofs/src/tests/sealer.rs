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

//! Sealed frames: they open in order, under their own direction's key,
//! unchanged; anything else is refused, and after one refusal the opener
//! refuses everything.

use nsh_crypto::{CryptoError, Opener, Sealer, Secret};

pub fn frame(s: &mut Sealer, inner: &[u8]) -> ([u8; 4], Vec<u8>) {
    let len = ((inner.len() + 16) as u32).to_le_bytes();
    let mut out = vec![0u8; inner.len() + 16];
    assert_eq!(s.seal(len, inner, &mut out), Ok(inner.len() + 16));
    (len, out)
}

pub fn open(o: &mut Opener, f: &([u8; 4], Vec<u8>)) -> Result<Vec<u8>, CryptoError> {
    let mut out = vec![0u8; f.1.len()];
    o.open(f.0, &f.1, &mut out).map(|n| out[..n].to_vec())
}

pub fn pair(k: u8) -> (Sealer, Opener) {
    (Sealer::new(Secret::new([k; 32])), Opener::new(Secret::new([k; 32])))
}

#[test]
fn frames_open_in_order_and_the_same_bytes_seal_differently_each_time() {
    let (mut s, mut o) = pair(1);
    let a = frame(&mut s, b"\x30ls");
    let b = frame(&mut s, b"\x30ls");
    assert_ne!(a.1, b.1, "each frame has its own nonce");
    assert_eq!(open(&mut o, &a), Ok(b"\x30ls".to_vec()));
    assert_eq!(open(&mut o, &b), Ok(b"\x30ls".to_vec()));
}

#[test]
fn a_frame_out_of_order_is_refused_and_ends_the_opener() {
    let (mut s, mut o) = pair(2);
    let a = frame(&mut s, b"\x30a");
    let b = frame(&mut s, b"\x30b");
    assert_eq!(open(&mut o, &b), Err(CryptoError::Open));
    assert_eq!(open(&mut o, &a), Err(CryptoError::Open), "dead after one refusal");
}
