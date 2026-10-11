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

//! Storing to a Name: the value is converted to the type the Name already
//! holds (section 19.3.5.8); CopyObject replaces it instead.

use super::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn write_named(
        &mut self,
        aml: &mut Aml,
        id: Id,
        v: Object,
        at: u32,
        copy: bool,
    ) -> Result<(), Fault> {
        let kind = aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?.kind;
        match kind {
            Kind::Name => {}
            // Field units and buffer fields write through their region or
            // buffer, which arrives with unit F 4.
            Kind::FieldUnit(_) | Kind::BufferField => {
                return Err(self.fault(FaultKind::Unsupported, at))
            }
            _ => return Err(self.fault(FaultKind::Type, at)),
        }
        let v = self.deref(aml, v, at)?;
        if copy {
            let v = self.copy(&v, at)?;
            *aml.rt.value(id) = Some(v);
            return Ok(());
        }
        let w = self.width(aml, 0);
        let new = match self.name_value(aml, id, at)? {
            Object::Integer(_) => v.as_integer(w).map(Object::Integer),
            Object::String(_) => v.as_text(w).map(|s| Object::string(&s)),
            // A buffer keeps its length: a shorter value is zero-filled, a
            // longer one cut, and the bytes change in place so references
            // into the buffer see them.
            Object::Buffer(b) => {
                let bytes = v.as_bytes(w).ok_or_else(|| self.fault(FaultKind::Type, at))?;
                let mut b = b.borrow_mut();
                let n = b.len().min(bytes.len());
                b[..n].copy_from_slice(&bytes[..n]);
                b[n..].iter_mut().for_each(|x| *x = 0);
                return Ok(());
            }
            _ => Some(self.copy(&v, at)?),
        };
        let new = new.ok_or_else(|| self.fault(FaultKind::Type, at))?;
        self.charge(new.size() as u64, at)?;
        *aml.rt.value(id) = Some(new);
        Ok(())
    }
}
