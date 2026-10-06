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

//! Enroll a kernel image and build the trailer that carries its path. The same
//! padding, tree and trailer the build side produces, from the same crate.

use super::constants::{DEPTH, LEAVES, PAD_SEED};
use super::context::kernel_context;
use nonos_attest_path::{digest_to_bytes, leaf, pad_leaf, Kind, Poseidon, Tree};

/// Enroll a kernel image in slot 0 of a padded tree at the gate depth. Returns the
/// serialized root and the kernel's trailer, or a zero root and an empty trailer,
/// which every gate refuses, if the tree cannot be built.
pub fn enroll_kernel(kernel_bytes: &[u8]) -> ([u8; 32], Vec<u8>) {
    let h = Poseidon::new();
    let Some(kernel) = leaf(&h, Kind::Kernel, &kernel_context(kernel_bytes)) else {
        return ([0; 32], Vec::new());
    };
    let mut leaves: Vec<_> = (0..LEAVES as u32).map(|i| pad_leaf(&h, &PAD_SEED, i)).collect();
    leaves[0] = kernel;
    let Some(tree) = Tree::commit(&h, &leaves, DEPTH) else {
        return ([0; 32], Vec::new());
    };
    match (tree.root(), tree.trailer(0)) {
        (Some(root), Some(trailer)) => (digest_to_bytes(&root), trailer),
        _ => ([0; 32], Vec::new()),
    }
}
