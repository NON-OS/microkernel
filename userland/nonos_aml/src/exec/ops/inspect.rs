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

//! SizeOf and ObjectType (sections 19.6.127 and 19.6.96), and a path read
//! from a string.

use crate::exec::place::Place;
use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::object::{type_of, Object, ObjectType};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn size_of(
        &mut self,
        aml: &mut Aml,
        t: u16,
        a: u32,
        at: u32,
    ) -> Result<Object, Fault> {
        let v = self.value(aml, t, a)?;
        let n = match &v {
            Object::String(b) | Object::Buffer(b) => b.borrow().len(),
            Object::Package(p) => p.borrow().len(),
            _ => return Err(self.fault(FaultKind::Type, at)),
        };
        Ok(Object::Integer(n as u64))
    }

    /// A named object's type comes from its kind, and a Name's from its value;
    /// a reference reports what it refers to.
    pub(crate) fn object_type(
        &mut self,
        aml: &mut Aml,
        t: u16,
        a: u32,
        at: u32,
    ) -> Result<Object, Fault> {
        let ty = match self.place(aml, t, a)? {
            Place::Debug => ObjectType::Debug,
            Place::Null => ObjectType::Uninit,
            Place::Named(id) => self.named_type(aml, id, at)?,
            place => {
                let r = self.reference(aml, place, at)?;
                let v = self.read_ref(aml, &r, at)?;
                match self.deref(aml, v, at)? {
                    Object::Named(id, _) => self.named_type(aml, id, at)?,
                    v => v.object_type(),
                }
            }
        };
        Ok(Object::Integer(ty as u64))
    }

    fn named_type(&mut self, aml: &mut Aml, id: Id, at: u32) -> Result<ObjectType, Fault> {
        match aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?.kind {
            Kind::Name => Ok(self.name_value(aml, id, at)?.object_type()),
            kind => Ok(type_of(kind)),
        }
    }
}
