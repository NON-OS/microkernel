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

//! An alias outlives the table that made it only as long as its object
//! does. An alias holds its object's slot, and a freed slot is reused: left
//! in place, it would name whatever object takes the slot next. So when an
//! object goes, every alias naming it, and every alias naming those, goes
//! too, and the alias's name resolves to nothing.

use super::{Id, Kind, Namespace};
use alloc::vec::Vec;

impl Namespace {
    pub(super) fn drop_aliases_to(&mut self, gone: &mut Vec<Id>) {
        let names_gone = |ns: &Self, a: Id, gone: &[Id]| {
            let kind = ns.get(a).map(|e| e.kind);
            matches!(kind, Some(Kind::Alias(t)) if gone.contains(&t))
        };
        while let Some(a) = (0..self.len() as Id).find(|a| names_gone(self, *a, gone)) {
            self.remove_into(a, gone);
        }
    }
}
