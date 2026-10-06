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

use crate::leaf::{leaf, Kind};
use crate::params::Digest;
use crate::poseidon::Poseidon;
use crate::trailer::{bytes_to_digest, parse, Path};

/*
 * The whole gate. Rebuild the leaf from the kind and the context the caller
 * computed from what it is about to run and grant, fold it up the path, and
 * compare with the root. A zero root was never enrolled and admits nothing. Any
 * malformed byte is a refusal, never a panic: every read is checked.
 */
#[must_use = "an image must not run unless its path folds to the root"]
pub fn verify(root: &[u8; 32], depth: usize, kind: Kind, ctx: &[u8], trailer: &[u8]) -> bool {
    if *root == [0u8; 32] || kind == Kind::Pad {
        return false;
    }
    let (Some(want), Some(path)) = (bytes_to_digest(root), parse(trailer, depth)) else {
        return false;
    };
    let h = Poseidon::new();
    let Some(start) = leaf(&h, kind, ctx) else {
        return false;
    };
    fold(&h, start, &path) == Some(want)
}

/// The node at the top of the path, or `None` on any non-canonical sibling.
pub(crate) fn fold(h: &Poseidon, start: Digest, path: &Path<'_>) -> Option<Digest> {
    let mut node = start;
    for k in 0..path.depth {
        let sib = path.sibling(k)?;
        node = if path.right(k)? { h.compress(&sib, &node) } else { h.compress(&node, &sib) };
    }
    Some(node)
}
