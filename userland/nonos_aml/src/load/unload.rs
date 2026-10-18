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

//! Unload (section 19.6.146): the objects a table loaded at run time defined
//! are removed with everything under them.

use super::Aml;
use crate::namespace::{Id, Origin};
use alloc::vec::Vec;

impl Aml {
    /// Removes every object table `t` defined; the DSDT cannot be unloaded.
    /// The table's bytes stay, unreachable, so any parse already made from
    /// them stays valid. Returns false for the DSDT or a table not loaded.
    pub fn unload(&mut self, t: u16) -> bool {
        if t == 0 || t as usize >= self.tables.len() {
            return false;
        }
        let mine: Vec<Id> = (0..self.ns.len() as Id)
            .filter(|id| matches!(self.ns.get(*id), Some(e) if matches!(e.origin, Origin::Table { table, .. } if table == t)))
            .collect();
        for id in mine {
            for gone in self.ns.remove(id) {
                self.rt.forget(gone);
            }
        }
        true
    }
}
