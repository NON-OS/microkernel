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

use crate::leaf::{leaf, pad_leaf, Kind};
use crate::poseidon::Poseidon;
use crate::trailer::digest_to_bytes;
use crate::tree::Tree;

pub const DEPTH: usize = 8;
pub const EPOCH: u64 = 1;
pub const PAD_SEED: [u8; 32] = [0x5a; 32];

/// The capsule spawn gate's context: measurement, capability word, epoch.
pub fn capsule_ctx(image: &[u8], caps: u64, epoch: u64) -> Vec<u8> {
    let mut c = blake3::hash(image).as_bytes().to_vec();
    c.extend_from_slice(&caps.to_be_bytes());
    c.extend_from_slice(&epoch.to_be_bytes());
    c
}

/// The bootloader's and secops's kernel context: measurement, boot epoch.
pub fn kernel_ctx(image: &[u8], epoch: u64) -> Vec<u8> {
    let mut c = blake3::hash(image).as_bytes().to_vec();
    c.extend_from_slice(&epoch.to_be_bytes());
    c
}

/// Enroll `slots` at `depth`, padding the rest, as the enroll tool does.
pub fn enroll(slots: &[(Kind, Vec<u8>)], depth: usize) -> ([u8; 32], Tree) {
    let h = Poseidon::new();
    let mut leaves: Vec<_> = slots.iter().map(|(k, c)| leaf(&h, *k, c).expect("leaf")).collect();
    for i in leaves.len()..1 << depth {
        leaves.push(pad_leaf(&h, &PAD_SEED, i as u32));
    }
    let tree = Tree::commit(&h, &leaves, depth).expect("tree");
    (digest_to_bytes(&tree.root().expect("root")), tree)
}

/// A capsule at caps 0x7 in slot 0 and a kernel in slot 1, at the gates' depth.
pub fn policy() -> ([u8; 32], Tree, Vec<u8>, Vec<u8>) {
    let (cap, kernel) = (b"capsule image".to_vec(), b"kernel image".to_vec());
    let slots = [
        (Kind::Capsule, capsule_ctx(&cap, 0x7, EPOCH)),
        (Kind::Kernel, kernel_ctx(&kernel, EPOCH)),
    ];
    let (root, tree) = enroll(&slots, DEPTH);
    (root, tree, cap, kernel)
}
