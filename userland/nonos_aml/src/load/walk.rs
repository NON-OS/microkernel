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

//! Walking a TermList while its table loads.

use super::{Aml, LoadExec};
use crate::namespace::{Id, NoteKind};
use alloc::vec::Vec;

impl Aml {
    /// Loads every object of the TERMS node `terms` into `scope`. A failure
    /// in one object is noted and the walk goes on, so one bad definition
    /// does not take the rest of the table with it.
    pub(crate) fn walk(&mut self, table: u16, terms: u32, scope: Id, exec: &mut impl LoadExec) {
        let kids: Vec<u32> = self.tables[table as usize].arena.kids(terms).to_vec();
        for node in kids {
            self.define(table, node, scope, exec);
        }
    }

    /// Runs a load-time If and loads the TermList it chooses.
    pub(super) fn branch(&mut self, table: u16, node: u32, scope: Id, exec: &mut impl LoadExec) {
        let kids = self.kids(table, node);
        match exec.condition(self, table, scope, node) {
            Ok(true) => self.walk(table, kids[1], scope, exec),
            Ok(false) => {
                if let Some(els) = kids.get(2) {
                    let body = self.kids(table, *els)[0];
                    self.walk(table, body, scope, exec);
                }
            }
            Err(e) => self.notes.push(NoteKind::Exec, table, e.at),
        }
    }
}
