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

//! Whether a pid holds the claim on a PCI function: the ACPI capsule asks
//! before it moves a device between D-states, so only the device's own
//! driver can power it down or up.

use super::super::{claim, pci_index};

/// Asked for pid 0: whether anyone holds it.
pub const ANYONE: u32 = 0;

/// None when segment 0 has no such function (only it is enumerated).
pub fn holds(pid: u32, bdf: (u16, u8, u8, u8)) -> Option<bool> {
    let (seg, bus, dev, func) = bdf;
    let h = pci_index::all().into_iter().find(|h| {
        let a = h.address;
        seg == 0 && a.segment == 0 && (a.bus, a.device, a.function) == (bus, dev, func)
    })?;
    Some(claim::lookup(h.device_id).is_some_and(|c| pid == ANYONE || c.pid == pid))
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionError {
    /// No PCI function has that device id.
    NotPci,
    /// The pid asking does not hold the device's claim.
    NotHolder,
}

/// The PCI function behind a broker device id: drivers know their device
/// by id, never by bus number. For a non-zero pid, only its holder may ask.
pub fn function_of_device(pid: u32, device_id: u64) -> Result<(u16, u8, u8, u8), FunctionError> {
    let h = pci_index::lookup(device_id).ok_or(FunctionError::NotPci)?;
    let holder = claim::lookup(device_id).map(|c| c.pid);
    if pid != ANYONE && holder != Some(pid) {
        return Err(FunctionError::NotHolder);
    }
    let a = h.address;
    Ok((a.segment, a.bus, a.device, a.function))
}
