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

//! The tree itself: entries in a vector, and an index from (parent, NameSeg)
//! to child.

use super::{Entry, Id, Kind, Origin, Predefined, Seg, ROOT};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Namespace {
    pub(super) entries: Vec<Entry>,
    pub(super) index: BTreeMap<(Id, Seg), Id>,
}

impl Namespace {
    /// An empty namespace holding only the root, before the predefined
    /// objects are installed.
    pub(super) fn bare() -> Self {
        let root = Entry {
            seg: *b"\\___",
            parent: ROOT,
            kind: Kind::Scope,
            origin: Origin::Predefined(Predefined::Root),
            first: None,
            last: None,
            next: None,
        };
        Self { entries: alloc::vec![root], index: BTreeMap::new() }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn get(&self, id: Id) -> Option<&Entry> {
        self.entries.get(id as usize)
    }

    pub fn child(&self, parent: Id, seg: Seg) -> Option<Id> {
        self.index.get(&(parent, seg)).copied()
    }

    /// The children of `parent`, in the order they were defined.
    pub fn children(&self, parent: Id) -> impl Iterator<Item = Id> + '_ {
        let first = self.get(parent).and_then(|e| e.first);
        core::iter::successors(first, move |id| self.entries[*id as usize].next)
    }
}
