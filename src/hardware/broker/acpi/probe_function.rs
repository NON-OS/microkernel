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

//! One PCI function looked at again after the boot scan: firmware left it
//! powered off, so the scan saw nothing there, and the ACPI capsule has
//! run its _PS0. Only that one function is read. If it answers it joins
//! the device table through the boot scan's own record build; if its BARs
//! were never assigned it joins marked unusable, since NONOS assigns none.
//! An IOMMU in service gives it no context entry: its DMA is denied until
//! a driver claims it and the claim confines it, as for any device. One
//! whose registers fall in a departed device's fenced window joins marked
//! unusable too, until the space that may still map it is gone.

use super::super::mmio::fence::fenced_by;
use super::super::{pci_index, table};
use super::late_bars::unassigned;
use crate::drivers::pci::manager::probe_function;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum ProbeError {
    /// The function is in the table already.
    Present,
    /// It still does not answer.
    Absent,
}

/// Segment 0 only: it is the only one enumerated.
pub fn probe_late(bus: u8, dev: u8, func: u8) -> Result<u64, ProbeError> {
    let known = pci_index::all().into_iter().any(|h| {
        let a = h.address;
        a.segment == 0 && (a.bus, a.device, a.function) == (bus, dev, func)
    });
    if known {
        return Err(ProbeError::Present);
    }
    let found = probe_function(bus, dev, func).ok_or(ProbeError::Absent)?;
    let bars: Vec<(u64, u64)> = found
        .bars
        .iter()
        .filter(|b| b.is_present())
        .map(|b| {
            (b.size(), b.address().map(|a| a.as_u64()).or(b.port().map(u64::from)).unwrap_or(0))
        })
        .collect();
    let mem = found.bars.iter().filter_map(|b| Some((b.address()?.as_u64(), b.size())));
    let fenced = mem.clone().find_map(|(base, size)| fenced_by(base, size));
    let disabled = unassigned(&bars) || fenced.is_some();
    let id = table::add_pci_function(&found, disabled);
    super::numa_refresh::refresh();
    super::probe_say::say((bus, dev, func), &found, id, disabled, fenced);
    Ok(id)
}
