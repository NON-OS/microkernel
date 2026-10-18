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

//! Adding one entry, and what a name already taken means.

use super::Aml;
use crate::grammar::NamePath;
use crate::namespace::{Id, Kind, NoteKind, Origin};

impl Aml {
    /// Adds the object `path` names from `scope` and returns where its own
    /// TermList, if it has one, should be loaded.
    ///
    /// A name defined twice in one scope is an error (section 5.3); the first
    /// definition stands and the second is noted. When both are containers
    /// (a Device defined again over a Device, or over \_SB) the second's
    /// TermList is still loaded into the first, so no object either table
    /// defines inside it is lost; anything else defined twice is dropped.
    pub(super) fn create(
        &mut self,
        table: u16,
        at: u32,
        scope: Id,
        path: NamePath,
        kind: Kind,
        origin: Origin,
    ) -> Option<Id> {
        let bytes = &self.tables[table as usize].bytes;
        let Some((parent, seg)) = self.ns.target(scope, bytes, path) else {
            self.notes.push(NoteKind::ParentMissing, table, at);
            return None;
        };
        match self.ns.add(parent, seg, kind, origin) {
            Ok(id) => Some(id),
            Err(existing) => {
                self.notes.push(NoteKind::Redefined, table, at);
                let was = self.ns.get(existing).map(|e| e.kind);
                (kind.is_container() && was.is_some_and(|k| k.is_container())).then_some(existing)
            }
        }
    }
}
