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

//! Buffer fields (CreateBitField to CreateQWordField and CreateField,
//! sections 19.6.18 to 19.6.23): bits of a buffer object, resolved when the
//! Create operator runs, read and written in place.

use crate::exec::{BufferField, Fault, FaultKind, Frame, Host, Machine};
use crate::load::Aml;
use crate::namespace::{Id, Origin};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn buffer_field(
        &mut self,
        aml: &mut Aml,
        id: Id,
        at: u32,
    ) -> Result<BufferField, Fault> {
        self.buffer_field_in(aml, id, at, true)
    }

    /// `fresh` as for `region_in`.
    pub(crate) fn buffer_field_in(
        &mut self,
        aml: &mut Aml,
        id: Id,
        at: u32,
        fresh: bool,
    ) -> Result<BufferField, Fault> {
        if let Some(f) = aml.rt.buffer_fields.get(&id) {
            return Ok(f.clone());
        }
        let e = *aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        let Origin::Table { table, node } = e.origin else {
            return Err(self.fault(FaultKind::Type, at));
        };
        if fresh {
            self.frames.push(Frame::new(table, e.parent));
        }
        let r = self.make_buffer_field(aml, table, node, at);
        if fresh {
            self.frames.pop();
        }
        let f = r?;
        aml.rt.buffer_fields.insert(id, f.clone());
        Ok(f)
    }
}
