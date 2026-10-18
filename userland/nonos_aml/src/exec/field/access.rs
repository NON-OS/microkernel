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

//! One access unit of a field: to its region, or through its index field.

use super::index_bank::{Ctx, Target};
use super::region_io::Io;
use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// Reads, or with `write` writes, the unit of `width` bytes at `byte`.
    pub(crate) fn unit_io(
        &mut self,
        aml: &mut Aml,
        ctx: &Ctx,
        byte: u64,
        width: u32,
        write: Option<u64>,
        at: u32,
    ) -> Result<u64, Fault> {
        match &ctx.target {
            Target::Region(rid, r) => self.region_io(aml, (*rid, r), Io { byte, width, write }, at),
            Target::Index { index, data } => {
                self.field_write(aml, *index, Object::Integer(byte), at)?;
                match write {
                    Some(v) => self.field_write(aml, *data, Object::Integer(v), at).map(|_| v),
                    None => {
                        let w = self.width(aml, 0);
                        let v = self.field_read(aml, *data, at)?;
                        v.as_integer(w).ok_or_else(|| self.fault(FaultKind::Type, at))
                    }
                }
            }
        }
    }
}
