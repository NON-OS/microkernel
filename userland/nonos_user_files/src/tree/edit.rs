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

//! Linking and unlinking names.

use crate::errno::{Errno, EEXIST, ENOSPC};
use crate::limits::MAX_NODES;
use crate::node::Node;

use super::Tree;

impl Tree {
    /// Gives `node` a number and links it into `dir` as `name`.
    pub fn add(&mut self, dir: u64, name: &[u8], mut node: Node) -> Result<u64, Errno> {
        if self.nodes.len() >= MAX_NODES {
            return Err(ENOSPC);
        }
        if self.child(dir, name).is_some() {
            return Err(EEXIST);
        }
        let ino = self.next_ino;
        self.next_ino += 1;
        node.parent = dir;
        node.name = name.to_vec();
        self.get_mut(dir)?.children.insert(name.to_vec(), ino);
        self.nodes.insert(ino, node);
        Ok(ino)
    }

    /// Takes `name` out of `dir`; the node stays in the tree, unlinked,
    /// for the caller to keep while it is open or to let go.
    pub fn unlink(&mut self, dir: u64, name: &[u8]) -> Result<u64, Errno> {
        let ino = self.get_mut(dir)?.children.remove(name).ok_or(crate::errno::ENOENT)?;
        self.get_mut(ino)?.linked = false;
        Ok(ino)
    }

    /// Links an existing node into `dir` as `name`, which must be free.
    pub fn relink(&mut self, ino: u64, dir: u64, name: &[u8]) -> Result<(), Errno> {
        self.get_mut(dir)?.children.insert(name.to_vec(), ino);
        let node = self.get_mut(ino)?;
        node.parent = dir;
        node.name = name.to_vec();
        node.linked = true;
        Ok(())
    }
}
