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
use crate::namespace::{Id, Kind, Origin};
use crate::object::Object;
use crate::region::Space;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// Marks `space`'s handler connected, then runs _REG(space, 1) in every
    /// scope that holds a region of that space and has a _REG, in the order
    /// the regions were defined. A _REG that faults is reported and the rest
    /// still run, so one broken device does not keep the others' regions
    /// unannounced.
    pub fn connect(&mut self, aml: &mut Aml, space: Space) -> Vec<(Id, Fault)> {
        aml.rt.connected |= space.bit();
        let mut scopes: Vec<Id> = Vec::new();
        for id in 0..aml.ns.len() as Id {
            let Some(e) = aml.ns.get(id) else { continue };
            let (Kind::OpRegion, Origin::Table { table, node }) = (e.kind, e.origin) else {
                continue;
            };
            if aml.imm(table, aml.kid(table, node, 1)) as u8 == space.0
                && !scopes.contains(&e.parent)
            {
                scopes.push(e.parent);
            }
        }
        let mut failed = Vec::new();
        for scope in scopes {
            let Some(reg) = aml.ns.child(scope, *b"_REG") else { continue };
            let args = alloc::vec![Object::Integer(space.0 as u64), Object::Integer(1)];
            self.frames.push(Frame::new(0, scope));
            let r = self.read_named(aml, reg, args, 0);
            self.frames.pop();
            if let Err(f) = r {
                failed.push((reg, f));
            }
        }
        failed
    }
}
