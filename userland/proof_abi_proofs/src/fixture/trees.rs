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

//! A release's bootloader and kernel trees, built the enroll tool's way, and
//! a device registry with decoys around this device.

use nonos_attest_path::{digest_to_bytes, leaf, pad_leaf, Kind, Poseidon as GateHasher, Tree};
use nonos_device_attest::{commit, words_of, Path, BOOT_DEPTH};
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;

pub const DEPTH: usize = 4;
pub const SECRET: [u64; 4] = [0x1111, 0x2222, 0x3333, 0x4444];

pub fn path_of(tree: &Tree, index: usize) -> Path {
    let t = tree.trailer(index).expect("trailer");
    let at = |k: usize| words_of(t[9 + k * 32..][..32].try_into().expect("32 bytes"));
    let dirs = &t[9 + BOOT_DEPTH * 32..];
    Path {
        siblings: (0..BOOT_DEPTH).map(at).collect(),
        right: (0..BOOT_DEPTH).map(|k| dirs[k / 8] >> (k % 8) & 1 == 1).collect(),
    }
}

/// The root, the tree with `ctx` in slot 0, and that slot's context digest.
pub fn tree(kind: Kind, ctx: &[u8]) -> ([Fp; RATE], Tree, [u8; 32]) {
    let h = GateHasher::new();
    let mut leaves: Vec<_> =
        (0..1u32 << BOOT_DEPTH).map(|i| pad_leaf(&h, &[0x5A; 32], i)).collect();
    leaves[0] = leaf(&h, kind, ctx).expect("leaf");
    let t = Tree::commit(&h, &leaves, BOOT_DEPTH).expect("tree");
    let d = nonos_attest_path::context_digest(ctx).expect("digest");
    (words_of(&digest_to_bytes(&t.root().expect("root"))), t, d)
}

/// The registry root and this device's path, `ours` at `at`.
pub fn registry(ours: [Fp; RATE], at: usize) -> ([Fp; RATE], Path) {
    let h = Poseidon::new(5, [Fp::ZERO; RATE]);
    let mut level: Vec<_> =
        (0..1u64 << DEPTH).map(|i| commit(&[Fp::from_u64(900 + i); RATE])).collect();
    level[at] = ours;
    let (mut siblings, mut right, mut i) = (Vec::new(), Vec::new(), at);
    while level.len() > 1 {
        siblings.push(level[i ^ 1]);
        right.push(i & 1 == 1);
        level = level.chunks_exact(2).map(|p| h.compress(&p[0], &p[1])).collect();
        i >>= 1;
    }
    (level[0], Path { siblings, right })
}
