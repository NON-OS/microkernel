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

//! What an OperationRegion's and a DataTableRegion's definitions evaluate to.

use crate::exec::{Fault, FaultKind, Host, Machine, Region};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::region::Space;
use alloc::rc::Rc;

impl<H: Host> Machine<'_, H> {
    /// DataTableRegion (section 19.6.32): the table named by signature and
    /// OEM ids, served from the platform's copy.
    pub(crate) fn data_region(
        &mut self,
        aml: &mut Aml,
        t: u16,
        k: &[u32],
        at: u32,
    ) -> Result<Region, Fault> {
        let mut ids = [alloc::vec::Vec::new(), alloc::vec::Vec::new(), alloc::vec::Vec::new()];
        for (i, slot) in ids.iter_mut().enumerate() {
            match self.value(aml, t, k[i + 1])? {
                Object::String(s) => *slot = s.borrow().clone(),
                _ => return Err(self.fault(FaultKind::Type, at)),
            }
        }
        let bytes = self
            .host
            .table(&ids[0], &ids[1], &ids[2])
            .ok_or_else(|| self.fault(FaultKind::Undefined, at))?;
        let len = bytes.len() as u64;
        Ok(Region {
            space: Space::SYSTEM_MEMORY,
            base: 0,
            len,
            pci: None,
            table: Some(Rc::new(bytes)),
        })
    }

    pub(crate) fn op_region(
        &mut self,
        aml: &mut Aml,
        t: u16,
        k: &[u32],
        scope: Id,
        at: u32,
    ) -> Result<Region, Fault> {
        let space = Space(aml.imm(t, k[1]) as u8);
        let (base, len) = (self.int(aml, t, k[2])?, self.int(aml, t, k[3])?);
        let pci = match space {
            Space::PCI_CONFIG => Some(self.pci_path(aml, scope, at)?),
            _ => None,
        };
        Ok(Region { space, base, len, pci, table: None })
    }
}
