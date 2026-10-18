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

//! An OperationRegion or DataTableRegion resolved the first time it is used,
//! or when a method defines it, while the locals it may be built from hold
//! what they hold then.

use crate::exec::{Fault, FaultKind, Frame, Host, Machine, Region};
use crate::load::Aml;
use crate::namespace::{Id, Kind, Origin};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn region(&mut self, aml: &mut Aml, id: Id, at: u32) -> Result<Region, Fault> {
        self.region_in(aml, id, at, true)
    }

    /// With `fresh`, the definition is evaluated in a frame of its own scope;
    /// without, in the running method's, whose arguments it may use.
    pub(crate) fn region_in(
        &mut self,
        aml: &mut Aml,
        id: Id,
        at: u32,
        fresh: bool,
    ) -> Result<Region, Fault> {
        if let Some(r) = aml.rt.regions.get(&id) {
            return Ok(r.clone());
        }
        let e = *aml.ns.get(id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
        let Origin::Table { table, node } = e.origin else {
            return Err(self.fault(FaultKind::Type, at));
        };
        let k = aml.kids(table, node);
        if fresh {
            self.frames.push(Frame::new(table, e.parent));
        }
        let r = match e.kind {
            Kind::OpRegion => self.op_region(aml, table, &k, e.parent, at),
            Kind::DataRegion => self.data_region(aml, table, &k, at),
            _ => Err(self.fault(FaultKind::Type, at)),
        };
        if fresh {
            self.frames.pop();
        }
        let r = r?;
        aml.rt.regions.insert(id, r.clone());
        Ok(r)
    }
}
