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

//! Measuring a capsule or kernel image to a Poseidon leaf. The bytes are absorbed
//! into a Poseidon sponge seven at a time (each block a canonical field element) and
//! the rate lanes are squeezed to a digest. The length is bound first so images of
//! different size cannot collide. That digest is the enrolled leaf: a policy root
//! commits to exactly the measured images, and an attestation proves membership of a
//! real measurement rather than an arbitrary secret.

use super::super::field::Fp;
use super::poseidon::{Poseidon, RATE, WIDTH};

/// The domain separator for the hybrid measurement, absorbed before the digest
/// so a hybrid leaf can never equal a direct leaf of the same bytes.
const HYBRID_DOMAIN: u64 = 0x4E4F_4E4F_5342_3348; // "NONOSB3H"

/// The hybrid measurement: BLAKE3 the image, then absorb only the 32-byte
/// digest into the sponge. This is what a caller should use to measure an
/// artifact of any real size.
///
/// The direct measurement below absorbs seven bytes per lane and permutes every
/// rate group, so it runs at about three megabytes a second: one permutation
/// per 28 bytes of artifact. BLAKE3 reads the same bytes three orders of
/// magnitude faster, and absorbing its 32-byte output costs a single
/// permutation regardless of image size. Measured on the crate's own study, the
/// hybrid reaches BLAKE3's own throughput and beats the direct path by more
/// than a thousand times at a megabyte.
///
/// The security question this raises is worth stating rather than assuming. The
/// hybrid binds a BLAKE3 digest where the direct path binds the bytes, so it
/// rests on BLAKE3 collision resistance. For an attestation whose proof context
/// already carries the artifact's BLAKE3 measurement, that assumption is
/// already load bearing and the hybrid removes redundant work rather than
/// adding a hypothesis. A caller whose context does not already commit to a
/// general purpose digest is making a different trade and should keep the
/// direct measurement.
///
/// The length is bound by BLAKE3 itself, and the domain separator keeps the two
/// measurement schemes disjoint: no image measures to the same leaf under both.
pub fn measure_capsule_hybrid(hasher: &Poseidon, image: &[u8]) -> [Fp; RATE] {
    let digest = blake3::hash(image);
    let bytes = digest.as_bytes();

    let mut state = [Fp::ZERO; WIDTH];
    state[0] = state[0] + Fp::from_u64(HYBRID_DOMAIN);
    // The 32-byte digest is four canonical field elements: each eight-byte
    // group is reduced, which is exact because 2^64 - p is smaller than p.
    for (lane, word) in bytes.chunks(8).enumerate() {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(word);
        state[lane + 1] = state[lane + 1] + Fp::from_u64(u64::from_le_bytes(buf));
    }
    state = hasher.permute(state);

    let mut out = [Fp::ZERO; RATE];
    out.copy_from_slice(&state[..RATE]);
    out
}

/// The Poseidon measurement of `image`: bind the length, absorb the bytes in
/// seven-byte little-endian blocks with one permutation per rate group, then squeeze
/// the rate lanes.
///
/// Kept for callers that must bind the bytes themselves rather than a digest of
/// them, and as the reference the hybrid is compared against. It is the slow
/// path by construction: see [`measure_capsule_hybrid`].
pub fn measure_capsule(hasher: &Poseidon, image: &[u8]) -> [Fp; RATE] {
    let mut state = [Fp::ZERO; WIDTH];
    state[0] = state[0] + Fp::from_u64(image.len() as u64);
    state = hasher.permute(state);

    let mut lane = 0usize;
    let mut i = 0usize;
    while i < image.len() {
        let take = core::cmp::min(7, image.len() - i);
        let mut buf = [0u8; 8];
        buf[..take].copy_from_slice(&image[i..i + take]);
        state[lane] = state[lane] + Fp::from_u64(u64::from_le_bytes(buf));
        lane += 1;
        if lane == RATE {
            state = hasher.permute(state);
            lane = 0;
        }
        i += 7;
    }
    if lane != 0 {
        state = hasher.permute(state);
    }

    let mut digest = [Fp::ZERO; RATE];
    digest.copy_from_slice(&state[..RATE]);
    digest
}
