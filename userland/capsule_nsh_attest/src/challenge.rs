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

//! The enrolment request's challenge and nonce text, as nsh-enrol reads
//! them: the challenge is SHA-256 of nsh-enrol's label, the nonce and
//! `H(host keys)`, so a document quoted over it names these keys; the nonce
//! is written in hex for `nsh-enrol body --nonce`. Pure, so
//! nsh_front_proofs holds it against nsh-enrol's own.

use alloc::vec::Vec;

use nsh_front_wire::KEYS_LEN;

const LABEL: &[u8] = b"nsh-enrol challenge\0";

/// The challenge an enrolment's document answers.
pub fn challenge(nonce: &[u8; 32], public: &[u8; KEYS_LEN]) -> [u8; 32] {
    let keys = nsh_crypto::key_digest(&public[..32], &public[32..]);
    nonos_hash::sha256(&[LABEL, &nonce[..], &keys[..]].concat())
}

/// The nonce file's text: the nonce in lowercase hex, newline ended.
pub fn nonce_text(nonce: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(65);
    crate::said::hex(&mut out, nonce);
    out.push(b'\n');
    out
}
