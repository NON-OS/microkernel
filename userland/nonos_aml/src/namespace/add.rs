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
        let id = self.entries.len() as Id;
        let e = Entry { seg, parent, kind, origin, first: None, last: None, next: None };
        self.entries.push(e);
        self.index.insert((parent, seg), id);
        match self.entries[parent as usize].last.replace(id) {
            Some(prev) => self.entries[prev as usize].next = Some(id),
            None => self.entries[parent as usize].first = Some(id),
        }
        Ok(id)
    }
}
