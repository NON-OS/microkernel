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

//! A one-slot side tree: an honest enrollment of a single context under its own
//! root, proven as enrollment proves every slot.

use nonos_attest_path::{digest_to_bytes, leaf, pad_leaf, verify, Kind, Poseidon, Tree};

use crate::context::{DEPTH, LEAVES};
use crate::io::{die, os_random};
use crate::stark::prove_v4;

pub fn enroll_one(kind: Kind, ctx: &[u8]) -> Vec<u8> {
    let h = Poseidon::new();
    let seed = os_random();
    let mut leaves: Vec<_> = (0..LEAVES as u32).map(|i| pad_leaf(&h, &seed, i)).collect();
    leaves[0] = leaf(&h, kind, ctx).unwrap_or_else(|| die("side leaf"));
    let tree = Tree::commit(&h, &leaves, DEPTH).unwrap_or_else(|| die("side tree"));
    let root = digest_to_bytes(&tree.root().unwrap_or_else(|| die("side root")));
    let path = tree.trailer(0).unwrap_or_else(|| die("side path"));
    if !verify(&root, DEPTH, kind, ctx, &path) {
        die("side path does not fold");
    }
    prove_v4(&root, kind, ctx, &path).unwrap_or_else(|e| die(&format!("side proof: {e}")))
}
