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

//! Adding an object.

use super::{Entry, Id, Kind, Namespace, Origin, Seg};

impl Namespace {
    /// Adds an object under `parent`, or returns the one already there.
    pub fn add(&mut self, parent: Id, seg: Seg, kind: Kind, origin: Origin) -> Result<Id, Id> {
        if let Some(existing) = self.child(parent, seg) {
            return Err(existing);
        }
        let (first, last, next, live) = (None, None, None, true);
        let id = match self.free.pop() {
            Some(id) => {
                let gen = self.entries[id as usize].gen.wrapping_add(1);
                let e = Entry { seg, parent, kind, origin, gen, live, first, last, next };
                self.entries[id as usize] = e;
                id
            }
            None => {
                let e = Entry { seg, parent, kind, origin, gen: 0, live, first, last, next };
                self.entries.push(e);
                self.entries.len() as Id - 1
            }
        };
        self.index.insert((parent, seg), id);
        if self.journaling > 0 {
            self.journal.push(id);
        }
        match self.entries[parent as usize].last.replace(id) {
            Some(prev) => self.entries[prev as usize].next = Some(id),
            None => self.entries[parent as usize].first = Some(id),
        }
        Ok(id)
    }
}
