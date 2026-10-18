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

use super::{Entry, Id, Seg};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Namespace {
    pub(super) entries: Vec<Entry>,
    pub(super) index: BTreeMap<(Id, Seg), Id>,
    /// Slots of removed objects, reused before the vector grows.
    pub(super) free: Vec<Id>,
    /// What was added while a method ran, for removal when it returns.
    pub(super) journal: Vec<Id>,
    pub(super) journaling: u32,
}

impl Namespace {
    /// Slots in use or free; ids are below this.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    /// Whether `id` still holds the object it held at generation `gen`.
    pub fn live(&self, id: Id, gen: u32) -> bool {
        self.get(id).is_some_and(|e| e.gen == gen)
    }

    /// The object in slot `id`, if one lives there now.
    pub fn get(&self, id: Id) -> Option<&Entry> {
        self.entries.get(id as usize).filter(|e| e.live)
    }

    pub fn child(&self, parent: Id, seg: Seg) -> Option<Id> {
        self.index.get(&(parent, seg)).copied()
    }

    /// The children of `parent`, in the order they were defined.
    pub fn children(&self, parent: Id) -> impl Iterator<Item = Id> + '_ {
        let first = self.get(parent).and_then(|e| e.first);
        core::iter::successors(first, move |id| self.entries[*id as usize].next)
    }

    /// Stops recording what is added, for a table Load brings in while a
    /// method runs: what a table defines outlives the method. Returns what
    /// `journal_resume` restores.
    pub fn journal_pause(&mut self) -> u32 {
        core::mem::take(&mut self.journaling)
    }

    pub fn journal_resume(&mut self, saved: u32) {
        self.journaling = saved;
    }
}
