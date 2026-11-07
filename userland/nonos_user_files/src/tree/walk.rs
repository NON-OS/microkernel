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

//! Finding a node by its path's components.

use crate::errno::{Errno, EBUSY, ENOENT, ENOTDIR};
use crate::node::Kind;

use super::{Tree, ROOT};

impl Tree {
    /// The node at `parts`, the root for none.
    pub fn walk(&self, parts: &[&[u8]]) -> Result<u64, Errno> {
        let mut at = ROOT;
        for part in parts {
            let node = self.get(at)?;
            if node.kind != Kind::Dir {
                return Err(ENOTDIR);
            }
            at = *node.children.get(*part).ok_or(ENOENT)?;
        }
        Ok(at)
    }

    /// The directory `parts` names its last component in, and that name.
    /// The root has no parent: EBUSY, as for a mount point.
    pub fn parent<'a>(&self, parts: &[&'a [u8]]) -> Result<(u64, &'a [u8]), Errno> {
        let (last, dirs) = parts.split_last().ok_or(EBUSY)?;
        let dir = self.walk(dirs)?;
        if self.get(dir)?.kind != Kind::Dir {
            return Err(ENOTDIR);
        }
        Ok((dir, last))
    }
}
