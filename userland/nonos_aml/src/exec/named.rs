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

//! A Name's value: made from its definition the first time it is read, kept
//! in the runtime after that.

use super::{Fault, FaultKind, Frame, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Origin, Predefined};
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn name_value(&mut self, aml: &mut Aml, id: Id, at: u32) -> Result<Object, Fault> {
        if let Some(v) = aml.rt.value(id).clone() {
            return Ok(v);
        }
        let e = *aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        let p = aml.rt.personality;
        let v = match e.origin {
            Origin::Predefined(Predefined::Os) => Object::string(p.os),
            Origin::Predefined(Predefined::Rev) => Object::Integer(p.rev),
            Origin::Predefined(_) => return Err(self.fault(FaultKind::Type, at)),
            // A Name's value is a data object; names inside it resolve from
            // the scope the Name stands in.
            Origin::Table { table, node } => {
                self.frames.push(Frame::new(table, e.parent));
                let r = self.term(aml, table, aml.kid(table, node, 1));
                self.frames.pop();
                r?
            }
        };
        *aml.rt.value(id) = Some(v.clone());
        Ok(v)
    }
}
