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

//! Connecting a space's handler and running _REG (section 6.5.4).

use crate::exec::{Fault, Frame, Host, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::region::Space;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// Marks `space`'s handler connected, then announces it (`announce`).
    pub fn connect(&mut self, aml: &mut Aml, space: Space) -> Vec<(Id, Fault)> {
        aml.rt.connected |= space.bit();
        self.announce(aml, space)
    }

    /// Runs _REG(space, 1) once in every scope that holds a region of that
    /// space and has a _REG, in the order the regions were defined. A scope
    /// is marked before its _REG runs, so a _REG that loads a table is not
    /// run again from inside itself. A _REG that faults on its own is
    /// reported and the rest still run.
    pub(super) fn announce(&mut self, aml: &mut Aml, space: Space) -> Vec<(Id, Fault)> {
        let mut scopes: Vec<Id> = Vec::new();
        for (scope, s) in super::reg_loaded::regions(aml) {
            let done = aml.rt.announced.contains(&(scope, space.0));
            if s == space.0 && !done && !scopes.contains(&scope) {
                scopes.push(scope);
            }
        }
        let mut failed = Vec::new();
        // Once the budget refuses, no _REG can run: the scopes left, and
        // one cut midway, stay untold for the next connect or load.
        for scope in scopes {
            if self.cut.is_some() {
                break;
            }
            let Some(reg) = aml.ns.child(scope, *b"_REG") else { continue };
            aml.rt.announced.push((scope, space.0));
            let args = alloc::vec![Object::Integer(space.0 as u64), Object::Integer(1)];
            self.frames.push(Frame::new(0, scope));
            let r = self.read_named(aml, reg, args, 0);
            self.frames.pop();
            if self.cut.is_some() {
                aml.rt.announced.retain(|told| *told != (scope, space.0));
            }
            if let Err(f) = r {
                failed.push((reg, f));
            }
        }
        failed
    }
}
