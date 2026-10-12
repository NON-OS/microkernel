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

//! Fields that are not registers: SMBus, IPMI, GenericSerialBus and PCC
//! fields move a buffer through the handler in one transaction (section
//! 5.5.2.4.5 and on); GPIO fields go to `gpio`.

use super::index_bank::{Ctx, Target};
use crate::exec::{Fault, FaultKind, Host, Machine, RegionAccess};
use crate::load::Aml;
use crate::object::Object;
use crate::region::Space;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// Some(value) if the field was one of these and the access is done; a
    /// write gives the transaction's response back in the source buffer,
    /// which is how firmware reads a write's status.
    pub(crate) fn special(
        &mut self,
        aml: &mut Aml,
        ctx: &Ctx,
        write: Option<Object>,
        at: u32,
    ) -> Result<Option<Object>, Fault> {
        let Target::Region(rid, r) = &ctx.target else { return Ok(None) };
        if r.space != Space::GPIO && !r.space.is_transactional() {
            return Ok(None);
        }
        if aml.rt.connected & r.space.bit() == 0 {
            return Err(self.fault(FaultKind::NoHandler, at));
        }
        let conn = self.connection(aml, ctx, at)?;
        let u = ctx.unit;
        let w = self.width(aml, 0);
        if r.space == Space::GPIO {
            return self.gpio_access(aml, ctx, conn, write, at);
        }
        let mut data = match &write {
            Some(v) => self
                .deref(aml, v.clone(), at)?
                .as_bytes(w)
                .ok_or_else(|| self.fault(FaultKind::Type, at))?,
            None => Vec::new(),
        };
        let (space, region, base, len) = (r.space, *rid, r.base, r.len);
        let address = base + (u.bit_offset / 8) as u64;
        let a = RegionAccess { space, region, address, width: 0, pci: None, base, len };
        if !self.host.transaction(&a, u.access, conn.as_deref(), &mut data) {
            return Err(self.fault(FaultKind::Region, at));
        }
        self.charge(data.len() as u64, at)?;
        if let Some(Object::Buffer(b)) = write {
            *b.borrow_mut() = data.clone();
        }
        Ok(Some(Object::buffer(data)))
    }
}
