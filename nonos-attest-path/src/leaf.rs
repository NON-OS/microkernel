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

use crate::field::Fp;
use crate::params::{Digest, RATE, WIDTH};
use crate::poseidon::Poseidon;

/// New with v3, so no v2 context digest reads as a v3 one.
const DOMAIN: &[u8] = b"NONOS-ATTEST-PATH-LEAF-v3";

/// "NONOSLV3": lane 0 of the leaf permutation, apart from the hybrid measurement's.
const LEAF_DOMAIN: u64 = 0x4E4F_4E4F_534C_5633;

const PAD_DOMAIN: &[u8] = b"NONOS-ATTEST-PATH-PAD-v3";

/// What a slot holds. The boot tree carries kernels and bootloaders, the
/// policy tree capsules. The anonymous proof pins the kind, so a bootloader
/// slot never opens as a kernel and neither ever opens as a capsule.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Kind {
    Kernel = 0,
    Capsule = 1,
    Pad = 2,
    Bootloader = 3,
}

/// The context digest: what a slot enrolls, before the kind is attached.
pub fn context_digest(ctx: &[u8]) -> Option<[u8; 32]> {
    let len = u32::try_from(ctx.len()).ok()?;
    let mut b = blake3::Hasher::new();
    b.update(DOMAIN);
    b.update(&len.to_le_bytes());
    b.update(ctx);
    Some(*b.finalize().as_bytes())
}

/*
 * The kind sits in its own lane of the permutation, not inside the BLAKE3 digest.
 * A gate rebuilds the leaf with the kind it is checking either way. The reason is
 * the anonymous proof, which takes the digest as a private witness and never sees
 * the context: with the kind in lane 5 the circuit pins it as a constant, so a
 * capsule slot cannot be opened as a kernel and a padding slot opens as nothing.
 */
pub fn leaf_of(h: &Poseidon, kind: Kind, digest: &[u8; 32]) -> Digest {
    let mut state = [Fp::ZERO; WIDTH];
    state[0] = Fp::from_u64(LEAF_DOMAIN);
    for (lane, word) in state[1..=RATE].iter_mut().zip(digest.chunks_exact(8)) {
        let mut w = [0u8; 8];
        w.copy_from_slice(word);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    state[RATE + 1] = Fp::from_u64(kind as u64);
    let out = h.permute(state);
    let mut d = [Fp::ZERO; RATE];
    d.copy_from_slice(&out[..RATE]);
    d
}

/*
 * The leaf is the whole context, not the image alone. A gate rebuilds the
 * context from what it is about to run and grant (the kernel's measurement and
 * boot epoch, or a capsule's measurement, its capability word and the policy
 * epoch), so a path only folds to the root when every one of those is the value
 * that was enrolled.
 */
pub fn leaf(h: &Poseidon, kind: Kind, ctx: &[u8]) -> Option<Digest> {
    Some(leaf_of(h, kind, &context_digest(ctx)?))
}

/*
 * A padding slot's digest comes from a seed the enroll tool draws once per tree
 * and records in the transcript, not from a constant. No gate asks for the Pad
 * kind and the circuit pins the kind, so this is depth: a pad digest is never a
 * value known before the tree was enrolled.
 */
pub fn pad_leaf(h: &Poseidon, seed: &[u8; 32], index: u32) -> Digest {
    let mut b = blake3::Hasher::new();
    b.update(PAD_DOMAIN);
    b.update(seed);
    b.update(&index.to_le_bytes());
    leaf_of(h, Kind::Pad, b.finalize().as_bytes())
}
