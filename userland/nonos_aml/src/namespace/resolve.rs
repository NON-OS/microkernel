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

//! Finding the object a NameString names.

use super::{Id, Namespace, Seg, ROOT};
use crate::grammar::NamePath;
use alloc::vec::Vec;

impl Namespace {
    /// The object `path` names from `scope`. A single NameSeg with no prefix
    /// is searched for in `scope`, then in each scope above it up to the root
    /// (section 5.3, the namespace search rules); any other path is followed
    /// exactly. Aliases are followed to what they name.
    pub fn resolve(&self, scope: Id, bytes: &[u8], path: NamePath) -> Option<Id> {
        let segs: Vec<Seg> = (0..path.count).map(|i| path.seg(bytes, i)).collect();
        if !path.root && path.up == 0 && segs.len() == 1 {
            let mut at = scope;
            loop {
                if let Some(id) = self.child(at, segs[0]) {
                    return self.unalias(id);
                }
                if at == ROOT {
                    return None;
                }
                at = self.get(at)?.parent;
            }
        }
        let start = self.start(scope, path.root, path.up)?;
        segs.iter().try_fold(start, |at, seg| self.unalias(self.child(at, *seg)?))
    }

    /// Where a definition of `path` in `scope` goes: its parent and its own
    /// NameSeg. The search rules do not apply to a definition; every scope on
    /// the way must already exist.
    pub fn target(&self, scope: Id, bytes: &[u8], path: NamePath) -> Option<(Id, Seg)> {
        let last = path.count.checked_sub(1)?;
        let start = self.start(scope, path.root, path.up)?;
        let parent =
            (0..last).try_fold(start, |at, i| self.unalias(self.child(at, path.seg(bytes, i))?))?;
        Some((parent, path.seg(bytes, last)))
    }
}
