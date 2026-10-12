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

//! Removing an object and everything under it: what a method made is gone
//! when it returns (section 19.6.82, objects created by a method).

use super::{Id, Namespace, ROOT};
use alloc::vec::Vec;

impl Namespace {
    /// Removes `id` and its subtree, and returns every id removed; their
    /// slots are reused later with a new generation. Removing the root, or a
    /// slot already free, does nothing.
    pub fn remove(&mut self, id: Id) -> Vec<Id> {
        let mut gone = Vec::new();
        self.remove_into(id, &mut gone);
        gone
    }

    fn remove_into(&mut self, id: Id, gone: &mut Vec<Id>) {
        if id == ROOT || self.get(id).is_none() {
            return;
        }
        let kids: Vec<Id> = self.children(id).collect();
        for k in kids {
            self.remove_into(k, gone);
        }
        let (parent, seg, next) = {
            let e = &self.entries[id as usize];
            (e.parent, e.seg, e.next)
        };
        let prev = self.children(parent).take_while(|c| *c != id).last();
        match prev {
            Some(p) => self.entries[p as usize].next = next,
            None => self.entries[parent as usize].first = next,
        }
        if self.entries[parent as usize].last == Some(id) {
            self.entries[parent as usize].last = prev;
        }
        self.index.remove(&(parent, seg));
        let e = &mut self.entries[id as usize];
        (e.live, e.next, e.first, e.last) = (false, None, None, None);
        self.free.push(id);
        gone.push(id);
    }

    /// Starts recording what is added, and returns where the record stands.
    pub fn journal_begin(&mut self) -> usize {
        self.journaling += 1;
        self.journal.len()
    }

    /// Stops recording and removes everything added since `mark`, newest
    /// first.
    pub fn journal_unwind(&mut self, mark: usize) -> Vec<Id> {
        self.journaling = self.journaling.saturating_sub(1);
        let made: Vec<Id> = self.journal.drain(mark.min(self.journal.len())..).collect();
        made.iter().rev().flat_map(|id| self.remove(*id)).collect()
    }
}
