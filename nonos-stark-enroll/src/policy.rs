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

//! One policy tree from a list of slots, with every trailer and the transcript an
//! auditor needs to rebuild the root.

use nonos_attest_path::{digest_to_bytes, leaf, pad_leaf, verify, Poseidon, Tree};

use crate::context::{DEPTH, LEAVES};
pub use crate::slot::Slot;

pub struct Policy {
    pub root: [u8; 32],
    pub trailers: Vec<Vec<u8>>,
}

/*
 * Leaves in slot order, the rest padded with Pad leaves drawn from the tree's
 * pad seed. Every slot leaves as a v4 trailer: its path, checked with the gate's
 * own fold, and a STARK proof of the same slot, checked with the gates'
 * verifier. A trailer that would be refused at boot is an error here instead.
 */
pub fn enroll(slots: &[Slot], pad_seed: &[u8; 32]) -> Result<Policy, String> {
    if slots.is_empty() || slots.len() > LEAVES {
        return Err(format!("{} slots, want 1 to {LEAVES}", slots.len()));
    }
    let h = Poseidon::new();
    let mut leaves: Vec<_> = (0..LEAVES as u32).map(|i| pad_leaf(&h, pad_seed, i)).collect();
    for (at, slot) in leaves.iter_mut().zip(slots) {
        let (kind, ctx) = slot.context();
        *at = leaf(&h, kind, &ctx).ok_or("leaf")?;
    }
    let tree = Tree::commit(&h, &leaves, DEPTH).ok_or("tree")?;
    let root = digest_to_bytes(&tree.root().ok_or("root")?);
    let mut jobs = Vec::with_capacity(slots.len());
    for (i, slot) in slots.iter().enumerate() {
        let path = tree.trailer(i).ok_or_else(|| format!("no trailer for slot {i}"))?;
        let (kind, ctx) = slot.context();
        if !verify(&root, DEPTH, kind, &ctx, &path) {
            return Err(format!("path {i} failed the gate's own check"));
        }
        jobs.push(crate::stark::Job { kind, ctx, path });
    }
    if paths_only() {
        return Ok(Policy { root, trailers: jobs.into_iter().map(|j| j.path).collect() });
    }
    let trailers = crate::stark::prove_all(&root, &jobs)?;
    Ok(Policy { root, trailers })
}

/*
 * NONOS_ENROLL_PATHS=1: every trailer is the path alone, already checked
 * above with the gate's own fold, and no STARK proof is made. Only a
 * development image's gates (nonos-dev-attest, the loader's dev-attest) take
 * such a trailer; a release refuses it. The seal sets it for a development
 * image and never for any other.
 */
pub fn paths_only() -> bool {
    std::env::var("NONOS_ENROLL_PATHS").is_ok_and(|v| v == "1")
}
