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

//! The entry points the capsule uses.

use super::{Fault, Frame, Host, Machine};
use crate::error::Error;
use crate::load::Aml;
use crate::namespace::{Id, ROOT};
use crate::object::Object;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// Evaluates the object at `id` with `args`: runs a method, or reads a
    /// Name, or returns a reference to anything else. Whatever happens, the
    /// mutexes the evaluation took are released before it returns.
    pub fn evaluate(&mut self, aml: &mut Aml, id: Id, args: Vec<Object>) -> Result<Object, Fault> {
        let scope = aml.ns.get(id).map_or(ROOT, |e| e.parent);
        let table = aml.tables.len().saturating_sub(1) as u16;
        self.frames.push(Frame::new(table, scope));
        let r = self.read_named(aml, id, args, 0).and_then(|v| self.deref(aml, v, 0));
        self.frames.clear();
        self.release_all(aml);
        r
    }

    /// Loads one table, running its load-time code in this evaluation.
    pub fn load_table(&mut self, aml: &mut Aml, bytes: Vec<u8>) -> Result<u16, Error> {
        let r = aml.load_table(bytes, self);
        self.release_all(aml);
        r
    }

    /// A mutex left held when an evaluation ends is a firmware bug; it is
    /// released so the next evaluation is not stuck behind it.
    fn release_all(&mut self, aml: &mut Aml) {
        for (m, _) in self.held.drain(..) {
            aml.rt.mutexes.remove(&m);
        }
    }
}
