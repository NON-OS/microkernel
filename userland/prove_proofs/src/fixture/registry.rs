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

//! The registrar's side: a registry of depth 4 with decoys around this
//! device, keyed on `ek_id` of each EK's TPM2B_PUBLIC as the capsule keys it.

use nonos_device_attest::{commit, ek_id, words_of, Registry};

pub const DEPTH: usize = 4;
pub const SECRET: [u64; 4] = [0x1111_2222, 0x3333_4444, 0x5555_6666, 0x7777_8888];

/// Four words as the 32 bytes `MkDeviceSecret` writes.
pub fn secret_of(words: [u64; 4]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (o, w) in out.chunks_exact_mut(8).zip(words) {
        o.copy_from_slice(&w.to_le_bytes());
    }
    out
}

/// The field's modulus, from the reduction itself: 2^64 - 1 reduces to
/// 2^64 - 1 - p.
pub fn field_p() -> u64 {
    u64::MAX - words_of(&[0xFF; 32])[0].to_u64()
}

/// An `MkEnroll` EK answer: the 34-byte name, then a TPM2B_PUBLIC whose
/// big-endian size covers its body.
pub fn answer(fill: u8) -> Vec<u8> {
    let body = vec![fill; 0x58];
    let mut a = vec![0x00, 0x0B];
    a.extend([fill; 32]);
    a.extend((body.len() as u16).to_be_bytes());
    a.extend(body);
    a
}

pub fn area(answer: &[u8]) -> &[u8] {
    &answer[34..]
}

/// Six decoys and `ours`, enrolled with `secret`'s commitment.
pub fn registry(ours: &[u8], secret: [u64; 4]) -> Registry {
    let mut r = Registry::new(DEPTH).expect("depth");
    for i in 0..6u64 {
        let decoy = secret_of([1000 + i, 7, 8, 9]);
        r.enroll(ek_id(&answer(i as u8)[34..]), commit(&words_of(&decoy))).expect("decoy");
    }
    r.enroll(ek_id(area(ours)), commit(&words_of(&secret_of(secret)))).expect("ours");
    r
}
