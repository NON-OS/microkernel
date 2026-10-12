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

//! Which PCI function a PCI_Config region belongs to (the PCI_Config space
//! of section 19.6.100): the _ADR of each device from the region's scope up
//! to the PCI host bridge, and the bridge's _SEG and _BBN (sections 6.1.1,
//! 6.5.5 and 6.5.6). The capsule turns the _ADRs into a bus number through
//! the bridges' secondary bus registers.

use crate::exec::{Fault, FaultKind, Host, Machine, PciPath};
use crate::load::Aml;
use crate::namespace::{Id, Kind, ROOT};
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// A scope with no host bridge above it is taken to be on segment 0,
    /// bus 0: the path is then what the _ADRs alone say.
    pub(crate) fn pci_path(&mut self, aml: &mut Aml, scope: Id, at: u32) -> Result<PciPath, Fault> {
        let mut adrs = Vec::new();
        let mut at_id = scope;
        while at_id != ROOT {
            let e = *aml.ns.get(at_id).ok_or_else(|| self.fault(FaultKind::Stale, at))?;
            if e.kind == Kind::Device && self.is_host_bridge(aml, at_id, at)? {
                adrs.reverse();
                let segment = self.child_int(aml, at_id, *b"_SEG", at)?.unwrap_or(0) as u16;
                let bus = self.child_int(aml, at_id, *b"_BBN", at)?.unwrap_or(0) as u8;
                return Ok(PciPath { segment, bus, adrs });
            }
            if let Some(adr) = self.child_int(aml, at_id, *b"_ADR", at)? {
                adrs.push(adr as u32);
            }
            at_id = e.parent;
        }
        adrs.reverse();
        Ok(PciPath { segment: 0, bus: 0, adrs })
    }
}
