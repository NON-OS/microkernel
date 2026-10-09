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

//! The statement's hashes, computed directly. The circuit constrains exactly
//! these, and the tests hold the two to each other and to the gates' own leaf.

use stark_proofs::crypto::stark::air::RATE;
use stark_proofs::crypto::stark::field::Fp;

use crate::domain::{DEVICE_DOMAIN, LEAF_DOMAIN, SCOPE_DOMAIN, TAG_DOMAIN};
use crate::params::hasher;

/// A 32-byte digest as four little-endian words, each reduced, as the gates
/// read it. Exact, since 2^64 - p is below p.
pub fn words_of(d: &[u8; 32]) -> [Fp; RATE] {
    let mut out = [Fp::ZERO; RATE];
    let (words, _) = d.as_chunks::<8>();
    for (o, w) in out.iter_mut().zip(words) {
        *o = Fp::from_u64(u64::from_le_bytes(*w));
    }
    out
}

/// A slot's leaf from its context digest: `nonos_attest_path::leaf_of`,
/// written as the one compression the circuit walks first.
pub fn leaf(kind: u64, d: &[u8; 32]) -> [Fp; RATE] {
    let w = words_of(d);
    hasher().compress(
        &[Fp::from_u64(LEAF_DOMAIN), w[0], w[1], w[2]],
        &[w[3], Fp::from_u64(kind), Fp::ZERO, Fp::ZERO],
    )
}

/// The device commitment, the only thing about `s` that leaves the machine.
pub fn commit(s: &[Fp; RATE]) -> [Fp; RATE] {
    hasher().compress(
        &[Fp::from_u64(DEVICE_DOMAIN), s[0], s[1], s[2]],
        &[s[3], Fp::ZERO, Fp::ZERO, Fp::ZERO],
    )
}

/// The device's tag under a scope: one per device per scope, unlinkable across
/// scopes without `s`.
pub fn tag(s: &[Fp; RATE], e: &[Fp; 2]) -> [Fp; RATE] {
    hasher().compress(&[Fp::from_u64(TAG_DOMAIN), s[0], s[1], s[2]], &[s[3], e[0], e[1], Fp::ZERO])
}

/*
 * The scope a verifier states: its identity and its window, never anything of
 * the device. The identity is length-prefixed so no two (id, window) pairs share
 * an encoding.
 */
pub fn scope(verifier_id: &[u8], window: u64) -> Option<[Fp; 2]> {
    let len = u32::try_from(verifier_id.len()).ok()?;
    let mut b = blake3::Hasher::new();
    b.update(SCOPE_DOMAIN);
    b.update(&len.to_le_bytes());
    b.update(verifier_id);
    b.update(&window.to_le_bytes());
    let w = words_of(b.finalize().as_bytes());
    Some([w[0], w[1]])
}
