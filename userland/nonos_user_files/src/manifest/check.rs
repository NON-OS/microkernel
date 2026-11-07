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

//! Whether decoded nodes are a tree: one root, every other node in a
//! directory under its own valid name, no loops, the person's folders and
//! nothing else at the root, and every
//! object number held by one place only.

use alloc::collections::BTreeSet;

use super::FIRST_ID;
use crate::node::Kind;
use crate::path::check as name_ok;
use crate::tree::{Tree, DOCUMENTS, DOWNLOADS, FOLDERS, ROOT};

/// Links each node into its directory as it checks it, so the tree it
/// passes is the tree the broker uses.
pub(super) fn tree(tree: &mut Tree, next_id: u64, freed: &[u64]) -> Option<()> {
    let root = tree.nodes.get(&ROOT)?;
    if root.kind != Kind::Dir || root.parent != 0 || !root.name.is_empty() {
        return None;
    }
    if next_id < FIRST_ID || tree.nodes.keys().any(|ino| *ino == 0 || *ino >= tree.next_ino) {
        return None;
    }
    let inos: alloc::vec::Vec<u64> = tree.nodes.keys().copied().filter(|i| *i != ROOT).collect();
    for ino in inos {
        let (parent, name) = {
            let n = tree.nodes.get(&ino)?;
            (n.parent, n.name.clone())
        };
        name_ok(&name).ok()?;
        let dir = tree.nodes.get_mut(&parent)?;
        if dir.kind != Kind::Dir || dir.children.insert(name, ino).is_some() {
            return None;
        }
    }
    if tree.nodes.keys().any(|ino| !tree.within(*ino, ROOT)) {
        return None;
    }
    // Documents and Downloads are in every manifest; the root holds only
    // folders, and only the ones FOLDERS names.
    for folder in [DOCUMENTS, DOWNLOADS] {
        tree.child(ROOT, folder)?;
    }
    for (name, ino) in &tree.nodes.get(&ROOT)?.children {
        if !FOLDERS.contains(&name.as_slice()) || tree.nodes.get(ino)?.kind != Kind::Dir {
            return None;
        }
    }
    let mut seen = BTreeSet::new();
    let held = tree.nodes.values().flat_map(|n| n.chunks.iter()).filter(|id| **id != 0);
    for id in held.chain(freed.iter()) {
        if *id < FIRST_ID || *id >= next_id || !seen.insert(*id) {
            return None;
        }
    }
    Some(())
}
