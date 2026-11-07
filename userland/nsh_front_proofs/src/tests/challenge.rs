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

//! The enrolment request's challenge is the one nsh-enrol computes, so the
//! document the machine quotes over it is the one `nsh-enrol body` takes;
//! and its nonce file reads back as the nonce `--nonce` is given.

use crate::challenge::{challenge, nonce_text};

fn sample(seed: u8) -> ([u8; 32], [u8; 1984]) {
    let mut nonce = [0u8; 32];
    let mut public = [0u8; 1984];
    for (i, b) in nonce.iter_mut().enumerate() {
        *b = seed.wrapping_mul(31).wrapping_add(i as u8);
    }
    for (i, b) in public.iter_mut().enumerate() {
        *b = seed.wrapping_add((i % 251) as u8);
    }
    (nonce, public)
}

#[test]
fn the_challenge_is_nsh_enrols() {
    for seed in [0u8, 1, 7, 200, 255] {
        let (nonce, public) = sample(seed);
        assert_eq!(challenge(&nonce, &public), nsh_enrol::challenge(&nonce, &public));
    }
}

#[test]
fn the_challenge_names_the_nonce_and_the_keys() {
    let (nonce, public) = sample(3);
    let base = challenge(&nonce, &public);
    let mut other = nonce;
    other[31] ^= 1;
    assert_ne!(challenge(&other, &public), base);
    for at in [0usize, 31, 32, 1983] {
        let mut keys = public;
        keys[at] ^= 0x80;
        assert_ne!(challenge(&nonce, &keys), base, "a key byte at {at}");
    }
}

#[test]
fn the_nonce_text_reads_back_as_nsh_enrol_reads_it() {
    let (nonce, _) = sample(9);
    let text = String::from_utf8(nonce_text(&nonce)).expect("ascii");
    assert!(text.ends_with('\n'));
    let back: [u8; 32] = nsh_enrol::hex::array(text.trim_end(), "nonce").expect("hex");
    assert_eq!(back, nonce);
    assert_eq!(text.len(), 65);
}
