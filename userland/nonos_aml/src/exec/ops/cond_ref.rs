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

//! CondRefOf (section 19.6.15).

use crate::exec::place::Place;
use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// True, with the reference stored to the target, if the name resolves or
    /// the local holds something; false, and the target untouched, if not.
    pub(crate) fn cond_ref_of(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        at: u32,
    ) -> Result<Object, Fault> {
        let (a, target) = (aml.kid(t, node, 0), aml.kid(t, node, 1));
        let found = match self.place(aml, t, a) {
            Ok(Place::Local(s) | Place::Arg(s)) if matches!(*s.borrow(), Object::Uninit) => None,
            Ok(p) => Some(self.reference(aml, p, at)?),
            Err(f) if f.kind == FaultKind::Undefined => None,
            Err(f) => return Err(f),
        };
        let w = self.width(aml, t);
        let Some(r) = found else { return Ok(Object::Integer(0)) };
        self.result(aml, t, target, Object::Ref(r), at)?;
        Ok(Object::Integer(w.ones()))
    }
}
