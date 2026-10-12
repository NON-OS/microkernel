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

//! A GPIO field (section 5.5.2.4.4): its bit offset is the first entry of its
//! connection's pin table and its length the number of pins.

use super::index_bank::Ctx;
use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn gpio_access(
        &mut self,
        aml: &mut Aml,
        ctx: &Ctx,
        conn: Option<Vec<u8>>,
        write: Option<Object>,
        at: u32,
    ) -> Result<Option<Object>, Fault> {
        let conn = conn.ok_or_else(|| self.fault(FaultKind::Type, at))?;
        let w = self.width(aml, 0);
        let value = match write {
            Some(v) => Some(
                self.deref(aml, v, at)?
                    .as_integer(w)
                    .ok_or_else(|| self.fault(FaultKind::Type, at))?,
            ),
            None => None,
        };
        let u = ctx.unit;
        let got = self.host.gpio(&conn, u.bit_offset, u.bit_len, value);
        got.map(|v| Some(Object::Integer(v))).ok_or_else(|| self.fault(FaultKind::Region, at))
    }
}
