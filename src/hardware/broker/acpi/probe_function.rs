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
//! run its _PS0. Only that one function is read; if it answers it joins
//! the device table like any other, so its driver can claim it.

use super::super::{pci_index, table};
use crate::drivers::pci::manager::probe_function;
use crate::sys::serial::Line;

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
    let id = table::add_pci_function(&found);
    Line::new()
        .str(b"[ACPI] pci ")
        .hex(u64::from(bus))
        .str(b":")
        .hex(u64::from(dev))
        .str(b".")
        .dec(u64::from(func))
        .str(b" answered after _PS0: ")
        .hex(u64::from(found.vendor_id))
        .str(b":")
        .hex(u64::from(found.device_id))
        .str(b" device=")
        .hex(id)
        .end();
    Ok(id)
}
