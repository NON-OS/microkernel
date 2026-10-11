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

//! Alias (section 19.6.4): a second name for an object that already exists.

use super::Aml;
use crate::namespace::{Id, Kind, NoteKind, Origin};

impl Aml {
    /// The source is found by the search rules from `scope`; an alias to
    /// nothing is noted and not made, so no name ever resolves to a hole.
    pub(super) fn alias(&mut self, table: u16, node: u32, scope: Id) {
        let kids = self.kids(table, node);
        let at = self.node(table, node).at;
        let (Some(source), Some(name)) =
            (self.name_of(table, kids[0]), self.name_of(table, kids[1]))
        else {
            return;
        };
        let bytes = &self.tables[table as usize].bytes;
        let Some(target) = self.ns.resolve(scope, bytes, source) else {
            return self.notes.push(NoteKind::AliasMissing, table, at);
        };
        let origin = Origin::Table { table, node };
        self.create(table, at, scope, name, Kind::Alias(target), origin);
    }
}
