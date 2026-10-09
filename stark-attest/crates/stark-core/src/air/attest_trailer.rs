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

//! Parsing and verifying a whole attestation trailer, the bytes a capsule
//! carries. The round and query counts are the kernel's, never the trailer's,
//! so a prover cannot weaken the low-degree test.

use super::super::field::{Fp, P};
use super::deserialize_ext::deserialize_proof_ext;
use super::merkle_membership::MerkleMembership;
use super::poseidon::{Poseidon, RATE};
use super::verify_ext::stark_verify_ext_blown_bound;
use crate::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS};
use alloc::vec::Vec;

/// The tag distinguishing a STARK trailer from the Curve25519 one.
pub const STARK_ATTEST_MAGIC: &[u8; 8] = b"NZKSTRK1";

fn read_fp(b: &[u8]) -> Option<Fp> {
    let v = u64::from_le_bytes(b.try_into().ok()?);
    (v < P).then(|| Fp::from_u64(v))
}

/// Verify a trailer against the trusted `root` and capsule `context`. Layout
/// after the magic: a depth byte, `depth * RATE` sibling field elements, the
/// direction bits, then the serialized proof. Total over any bytes.
#[must_use = "an attestation result must gate the spawn"]
pub fn verify_attestation_trailer(
    hasher: &Poseidon,
    log_rounds: u32,
    root: [Fp; RATE],
    n_queries: usize,
    blob: &[u8],
    context: &[u8],
) -> bool {
    if blob.len() < 9 || &blob[0..8] != STARK_ATTEST_MAGIC {
        return false;
    }
    let depth = blob[8] as usize;
    if depth == 0 {
        return false;
    }
    let dir_bytes = depth.div_ceil(8);
    let header = 9 + depth * RATE * 8 + dir_bytes;
    if blob.len() < header {
        return false;
    }

    // Siblings first, then the direction bitfield: the order the builder
    // emits and the order the kernel's spawn gate and the bootloader parse.
    // This function once read the two regions swapped, an emitter-verifier
    // differential nothing exercised because every deployed consumer carries
    // its own parse; the round-trip test below pins the order now.
    let sib = &blob[9..9 + depth * RATE * 8];
    let dir = &blob[9 + depth * RATE * 8..header];
    let directions: Vec<bool> = (0..depth).map(|i| (dir[i / 8] >> (i % 8)) & 1 == 1).collect();

    let mut siblings = Vec::with_capacity(depth);
    for level in 0..depth {
        let mut d = [Fp::ZERO; RATE];
        for (c, cell) in d.iter_mut().enumerate() {
            let off = (level * RATE + c) * 8;
            match read_fp(&sib[off..off + 8]) {
                Some(v) => *cell = v,
                None => return false,
            }
        }
        siblings.push(d);
    }

    // The deployed pipeline, exactly as the kernel's spawn gate and the
    // bootloader run it: the extension-field proof, the transcript grind, the
    // extra blowup, all from the parameter authority. This function once
    // routed into the retired base-field path, which no emitted trailer has
    // ever satisfied; the round-trip test below is what keeps this entry
    // point honest now.
    let Some(proof) = deserialize_proof_ext(&blob[header..]) else {
        return false;
    };
    let air = MerkleMembership::new(hasher.clone(), log_rounds, root, siblings, directions);
    stark_verify_ext_blown_bound(&air, &proof, n_queries, GRIND_BITS, EXTRA_BLOWUP_BITS, context)
}

#[cfg(test)]
mod round_trip {
    use super::super::attest_build::{build_attestation_trailer_from_set, MeasuredSet};
    use super::*;
    use crate::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};

    /// The builder and this verifier speak one wire format. They diverged
    /// once, directions and siblings swapped, and nothing noticed because
    /// every deployed consumer parses for itself; this test makes the crate
    /// notice.
    #[test]
    fn what_the_builder_emits_this_verifier_accepts() {
        let hasher = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
        let imgs: [&[u8]; 2] = [b"artifact alpha", b"artifact beta"];
        let mut padded: Vec<&[u8]> = imgs.to_vec();
        while padded.len() < 256 {
            padded.push(b"\x00STARK-ATTEST-RESERVED-SLOT-v1");
        }
        let set = MeasuredSet::commit(&hasher, &padded);
        let ctx = b"round-trip context";
        let trailer = build_attestation_trailer_from_set(
            &hasher,
            LOG_ROUNDS,
            &set,
            1,
            ctx,
            N_QUERIES,
            GRIND_BITS,
            EXTRA_BLOWUP_BITS,
        );
        assert!(
            verify_attestation_trailer(&hasher, LOG_ROUNDS, set.root(), N_QUERIES, &trailer, ctx),
            "the canonical verifier must accept what the builder emits"
        );
        let mut bad = trailer.clone();
        let last = bad.len() - 1;
        bad[last] ^= 1;
        assert!(
            !verify_attestation_trailer(&hasher, LOG_ROUNDS, set.root(), N_QUERIES, &bad, ctx),
            "one flipped bit must be refused"
        );
    }
}
