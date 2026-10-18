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

//! The named object an operator acts on: named directly, or passed in a
//! local or argument.

use crate::exec::place::Place;
use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::object::Object;
use core::mem::discriminant;

impl<H: Host> Machine<'_, H> {
    /// With `kind`, the object must be of that kind (a Mutex for Acquire, an
    /// Event for Wait); without, any named object will do.
    pub(crate) fn object_of(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        kind: Option<Kind>,
    ) -> Result<Id, Fault> {
        let at = aml.node(t, node).at;
        let id = match self.place(aml, t, node)? {
            Place::Named(id) => id,
            Place::Local(s) | Place::Arg(s) => match &*s.borrow() {
                Object::Named(id, gen) if aml.ns.live(*id, *gen) => *id,
                _ => return Err(self.fault(FaultKind::Type, at)),
            },
            _ => return Err(self.fault(FaultKind::Type, at)),
        };
        let e = aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        match kind {
            Some(k) if discriminant(&e.kind) != discriminant(&k) => {
                Err(self.fault(FaultKind::Type, at))
            }
            _ => Ok(id),
        }
    }
}
