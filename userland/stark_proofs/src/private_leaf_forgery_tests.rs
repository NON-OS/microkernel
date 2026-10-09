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

//! T2's counterexample, kept as a test so it cannot quietly move: under the
//! private-leaf gate a trailer proving an enrolled slot verifies under a
//! context picked for an image that was never enrolled. The gate the kernel
//! runs measures the image itself; nonos-attest-path's image_bind_tests hold
//! that it refuses the same forgery.

use crate::crypto::stark::air::{build_attestation_trailer_from_set, verify_membership_trailer};
use crate::crypto::stark::air::{MeasuredSet, Poseidon, RATE};
use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

const DEPTH: usize = 3;
const ROGUE: &[u8] = b"\x7fELF never enrolled";

fn context(image: &[u8], caps: u64) -> Vec<u8> {
    let mut ctx = blake3::hash(image).as_bytes().to_vec();
    ctx.extend_from_slice(&caps.to_be_bytes());
    ctx
}

fn set(h: &Poseidon) -> MeasuredSet {
    let imgs: Vec<Vec<u8>> =
        (0..1usize << DEPTH).map(|k| alloc::vec![0x7f, b'E', b'L', b'F', k as u8]).collect();
    let refs: Vec<&[u8]> = imgs.iter().map(|v| v.as_slice()).collect();
    MeasuredSet::commit(h, &refs)
}

#[test]
fn the_private_leaf_gate_accepts_the_forgery() {
    let h = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let (s, ctx) = (set(&h), context(ROGUE, 7));
    let forged = build_attestation_trailer_from_set(
        &h,
        LOG_ROUNDS,
        &s,
        2,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    let mut root = [0u8; 32];
    for (i, lane) in s.root().iter().enumerate() {
        root[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    let ok = verify_membership_trailer(
        &h,
        LOG_ROUNDS,
        root,
        DEPTH,
        &forged,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    assert!(ok, "the private-leaf forgery stopped verifying; T2's counterexample moved");
}
