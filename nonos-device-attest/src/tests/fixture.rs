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

//! A release and a device, built the way the enroll tool and a registrar would.

use nonos_attest_path::{
    digest_to_bytes, leaf as gate_leaf, pad_leaf, Kind, Poseidon as GateHasher, Tree,
};
use stark_proofs::crypto::stark::air::RATE;
use stark_proofs::crypto::stark::field::Fp;

use crate::native::{commit, scope, tag, words_of};
use crate::params::{hasher, BOOT_DEPTH};
use crate::statement::Statement;
use crate::witness::{Path, Slot, Witness};

pub const DEVICE_DEPTH: usize = 4;
pub const EPOCH: u64 = 1;

pub fn boot_ctx(image: &[u8]) -> Vec<u8> {
    let mut c = blake3::hash(image).as_bytes().to_vec();
    c.extend_from_slice(&EPOCH.to_be_bytes());
    c
}

/// The context digest the gate's leaf hashes, from the gate's own function.
pub fn digest(ctx: &[u8]) -> [u8; 32] {
    nonos_attest_path::context_digest(ctx).expect("context")
}

/// The path of `index` read out of the enroll tool's trailer, as the circuit
/// takes it.
fn path_of(tree: &Tree, index: usize, depth: usize) -> Path {
    let t = tree.trailer(index).expect("trailer");
    let mut siblings = Vec::with_capacity(depth);
    for k in 0..depth {
        let at = 9 + k * 32;
        siblings.push(words_of(t[at..at + 32].try_into().expect("32 bytes")));
    }
    let dirs = &t[9 + depth * 32..];
    let right = (0..depth).map(|k| dirs[k / 8] >> (k % 8) & 1 == 1).collect();
    Path { siblings, right }
}

fn to_fp(d: [u8; 32]) -> [Fp; RATE] {
    words_of(&d)
}

/// A tree of `kind` slots, the enroll tool's shape, with `ctxs` in the first
/// slots. Returns the root and the tree.
pub fn enrolled(kind: Kind, ctxs: &[Vec<u8>]) -> ([Fp; RATE], Tree) {
    let h = GateHasher::new();
    let mut leaves: Vec<_> =
        (0..1u32 << BOOT_DEPTH).map(|i| pad_leaf(&h, &[0x5a; 32], i)).collect();
    for (at, c) in leaves.iter_mut().zip(ctxs) {
        *at = gate_leaf(&h, kind, c).expect("leaf");
    }
    let tree = Tree::commit(&h, &leaves, BOOT_DEPTH).expect("tree");
    (to_fp(digest_to_bytes(&tree.root().expect("root"))), tree)
}

/// A registry of device commitments, `ours` at `index`.
pub fn registry(ours: [Fp; RATE], index: usize) -> ([Fp; RATE], Path) {
    let h = hasher();
    let n = 1usize << DEVICE_DEPTH;
    let mut level: Vec<[Fp; RATE]> = (0..n)
        .map(|i| {
            commit(&[
                Fp::from_u64(1000 + i as u64),
                Fp::from_u64(7),
                Fp::from_u64(8),
                Fp::from_u64(9),
            ])
        })
        .collect();
    level[index] = ours;
    let mut siblings = Vec::new();
    let mut right = Vec::new();
    let mut at = index;
    while level.len() > 1 {
        siblings.push(level[at ^ 1]);
        right.push(at & 1 == 1);
        level = level.as_chunks::<2>().0.iter().map(|[l, r]| h.compress(l, r)).collect();
        at >>= 1;
    }
    (level[0], Path { siblings, right })
}

pub struct Fixture {
    pub st: Statement,
    pub w: Witness,
    pub boot: Tree,
    pub kernel: Tree,
}

pub const SECRET: [u64; 4] = [0x1111, 0x2222, 0x3333, 0x4444];

pub fn fixture() -> Fixture {
    let bl_ctx = boot_ctx(b"bootloader.efi bytes");
    let k_ctx = boot_ctx(b"kernel image bytes");
    let (boot_root, boot) = enrolled(Kind::Bootloader, std::slice::from_ref(&bl_ctx));
    let (kernel_root, kernel) = enrolled(Kind::Kernel, std::slice::from_ref(&k_ctx));
    let s = SECRET.map(Fp::from_u64);
    let (device_root, device) = registry(commit(&s), 5);
    let e = scope(b"faucet.nonos.software", 20_000).expect("scope");
    let st = Statement {
        boot_root,
        kernel_root,
        device_root,
        device_depth: DEVICE_DEPTH,
        scope: e,
        context: [Fp::from_u64(42); RATE],
        tag: tag(&s, &e),
    };
    let w = Witness {
        secret: s,
        bootloader: Slot { digest: digest(&bl_ctx), path: path_of(&boot, 0, BOOT_DEPTH) },
        kernel: Slot { digest: digest(&k_ctx), path: path_of(&kernel, 0, BOOT_DEPTH) },
        device,
    };
    Fixture { st, w, boot, kernel }
}

pub fn path_at(tree: &Tree, index: usize) -> Path {
    path_of(tree, index, BOOT_DEPTH)
}

pub const ENTROPY: [u8; 64] = [0x33; 64];
