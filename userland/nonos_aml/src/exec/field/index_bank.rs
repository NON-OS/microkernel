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

//! A field unit's context: where its accesses go and under which rules.

use crate::exec::{Fault, FaultKind, Frame, Host, Machine, Region};
use crate::load::Aml;
use crate::namespace::{FieldUnit, Id, Kind, Origin};

/// Where a field unit's accesses go.
pub(crate) enum Target {
    Region(Id, Region),
    /// IndexField (section 19.6.63): the byte offset is written to the
    /// index field, then the data field is read or written.
    Index {
        index: Id,
        data: Id,
    },
}

pub(crate) struct Ctx {
    pub unit: FieldUnit,
    pub table: u16,
    pub scope: Id,
    pub flags: u8,
    pub target: Target,
    /// BankField (section 19.6.9): the bank field and the value that selects
    /// this field's bank, written before every access.
    pub bank: Option<(Id, u64)>,
}

impl<H: Host> Machine<'_, H> {
    pub(crate) fn field_ctx(&mut self, aml: &mut Aml, id: Id, at: u32) -> Result<Ctx, Fault> {
        let e = *aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        let (Kind::FieldUnit(unit), Origin::Table { table, .. }) = (e.kind, e.origin) else {
            return Err(self.fault(FaultKind::Type, at));
        };
        let k = aml.kids(table, unit.list);
        let flags = aml.imm(table, k[k.len() - 2]) as u8;
        self.frames.push(Frame::new(table, e.parent));
        let r = self.target(aml, table, unit.list, &k, at);
        self.frames.pop();
        let (target, bank) = r?;
        Ok(Ctx { unit, table, scope: e.parent, flags, target, bank })
    }
}
