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

//! A table's bytes, for Load.

use crate::exec::field::Io;
use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::object::Object;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// A table's bytes from an OperationRegion, read a byte at a time, or
    /// from a field unit or a Name's buffer.
    pub(crate) fn table_bytes(&mut self, aml: &mut Aml, id: Id, at: u32) -> Result<Vec<u8>, Fault> {
        let kind = aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?.kind;
        if kind == Kind::OpRegion {
            let r = self.region(aml, id, at)?;
            if r.len > aml.limits.max_table as u64 {
                return Err(self.fault(FaultKind::Alloc, at));
            }
            self.charge(r.len, at)?;
            let mut out = Vec::with_capacity(r.len as usize);
            for b in 0..r.len {
                let io = Io { byte: b, width: 1, write: None };
                out.push(self.region_io(aml, (id, &r), io, at)? as u8);
            }
            return Ok(out);
        }
        let v = self.read_named(aml, id, Vec::new(), at)?;
        match self.deref(aml, v, at)? {
            Object::Buffer(b) => Ok(b.borrow().clone()),
            _ => Err(self.fault(FaultKind::Type, at)),
        }
    }
}
