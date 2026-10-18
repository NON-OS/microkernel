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

//! Scope: loading into an object that already exists.

use super::{Aml, LoadExec};
use crate::namespace::{Id, NoteKind};

impl Aml {
    /// Scope opens an object that already exists (section 19.6.122); it
    /// defines nothing itself.
    pub(super) fn reopen(&mut self, table: u16, node: u32, scope: Id, exec: &mut impl LoadExec) {
        let kids = self.kids(table, node);
        let found = self
            .name_of(table, kids[0])
            .and_then(|path| self.ns.resolve(scope, &self.tables[table as usize].bytes, path));
        match found {
            Some(id) => self.walk(table, kids[1], id, exec),
            None => self.notes.push(NoteKind::ScopeMissing, table, self.node(table, node).at),
        }
    }
}
