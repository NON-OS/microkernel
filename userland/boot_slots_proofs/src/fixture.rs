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

//! A release as the enroll tool builds it: a boot tree with images enrolled in
//! its first slots, their v4 trailers, and a kernel file signed in the
//! loader's format with its trailer in the proof footer.

use nonos_attest_path::{
    boot_context, digest_to_bytes, encode_v4, leaf, pad_leaf, Kind, Poseidon, Tree,
};

pub const DEPTH: usize = 8;
pub const EPOCH: u64 = 1;
pub const HAS_ZK_PROOF: u16 = 1;

/// The tree of `kind` with `measured` enrolled in its first slots; its root.
pub fn tree(kind: Kind, measured: &[[u8; 32]]) -> ([u8; 32], Tree) {
    let h = Poseidon::new();
    let mut leaves: Vec<_> = (0..1u32 << DEPTH).map(|i| pad_leaf(&h, &[0x5a; 32], i)).collect();
    for (at, m) in leaves.iter_mut().zip(measured) {
        *at = leaf(&h, kind, &boot_context(m, EPOCH)).expect("leaf");
    }
    let tree = Tree::commit(&h, &leaves, DEPTH).expect("tree");
    (digest_to_bytes(&tree.root().expect("root")), tree)
}

/// The v4 trailer of slot `index`. The proof is a stand-in: a slot is read
/// from the path, and the STARK is the loader's to check.
pub fn trailer(tree: &Tree, kind: Kind, index: usize) -> Vec<u8> {
    encode_v4(kind, &tree.trailer(index).expect("path"), &[0xAB; 96]).expect("v4")
}

/// `kernel`, a stand-in signature and `proof`, then the 64-byte footer.
pub fn signed_file(kernel: &[u8], proof: &[u8], flags: u16, sig_alg: u8) -> Vec<u8> {
    let mut f = kernel.to_vec();
    let sig_at = f.len() as u32;
    f.extend_from_slice(&[0x5E; 64]);
    let proof_at = f.len() as u32;
    f.extend_from_slice(proof);
    let total = (f.len() + 64) as u64;
    f.extend_from_slice(b"NONOSIMG");
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&flags.to_le_bytes());
    f.extend_from_slice(&[1, sig_alg, 0, 0]);
    f.extend_from_slice(&total.to_le_bytes());
    for v in [0, kernel.len() as u32, sig_at, 64, proof_at, proof.len() as u32, 1, 0, 1, 0] {
        f.extend_from_slice(&v.to_le_bytes());
    }
    f
}

pub fn measure(kernel: &[u8]) -> [u8; 32] {
    *blake3::hash(kernel).as_bytes()
}
