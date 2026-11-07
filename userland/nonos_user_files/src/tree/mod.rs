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

//! The tree in RAM: every node by its number, each directory naming its own.

mod edit;
mod walk;

use alloc::collections::BTreeMap;

use crate::errno::{Errno, EIO};
use crate::node::{Kind, Node};

/// The root, which holds the person's folders and nothing else.
pub const ROOT: u64 = 1;
/// The folders a client sees; none can be removed or renamed.
pub const DOCUMENTS: &[u8] = b"Documents";
pub const DOWNLOADS: &[u8] = b"Downloads";
pub const MUSIC: &[u8] = b"Music";
pub const PICTURES: &[u8] = b"Pictures";
pub const VIDEOS: &[u8] = b"Videos";
/// Every folder, in the order a listing gives them. Documents and Downloads
/// are in every manifest; Music, Pictures and Videos came later, and a tree
/// committed before them gains them at its next start.
pub const FOLDERS: [&[u8]; 5] = [DOCUMENTS, DOWNLOADS, MUSIC, PICTURES, VIDEOS];

pub struct Tree {
    pub nodes: BTreeMap<u64, Node>,
    pub next_ino: u64,
}

impl Tree {
    /// A first start: the root and its empty folders.
    pub fn fresh(now: u64) -> Tree {
        let mut tree = Tree { nodes: BTreeMap::new(), next_ino: ROOT + 1 };
        tree.nodes.insert(ROOT, Node::new(Kind::Dir, 0o755, 0, b"", now));
        tree.ensure_folders(now);
        tree
    }

    /// Makes every folder that is missing: all of them on a first start,
    /// the later ones in a tree committed before them. Whether it made any.
    /// A tree already at MAX_NODES gains none, and stays as it is.
    pub fn ensure_folders(&mut self, now: u64) -> bool {
        let mut made = false;
        for name in FOLDERS {
            if self.child(ROOT, name).is_none() {
                let node = Node::new(Kind::Dir, 0o700, ROOT, name, now);
                made |= self.add(ROOT, name, node).is_ok();
            }
        }
        made
    }

    /// A node the tree holds. A number the tree itself gave out and does
    /// not find is a broken tree, never a client's mistake.
    pub fn get(&self, ino: u64) -> Result<&Node, Errno> {
        self.nodes.get(&ino).ok_or(EIO)
    }

    pub fn get_mut(&mut self, ino: u64) -> Result<&mut Node, Errno> {
        self.nodes.get_mut(&ino).ok_or(EIO)
    }

    pub fn child(&self, dir: u64, name: &[u8]) -> Option<u64> {
        self.nodes.get(&dir).and_then(|d| d.children.get(name).copied())
    }

    /// The root and its folders: nothing moves or removes them.
    pub fn is_fixed(&self, ino: u64) -> bool {
        ino == ROOT || self.nodes.get(&ino).is_some_and(|n| n.parent == ROOT)
    }

    /// Whether `ino` is `ancestor` or below it.
    pub fn within(&self, mut ino: u64, ancestor: u64) -> bool {
        // A tree's depth is at most its node count; a longer chain is a loop.
        for _ in 0..=self.nodes.len() {
            if ino == ancestor {
                return true;
            }
            match self.nodes.get(&ino) {
                Some(n) if n.parent != 0 => ino = n.parent,
                _ => return false,
            }
        }
        false
    }
}
