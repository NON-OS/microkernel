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

//! MkAcpiRestoreFunction (L64): after the ACPI owner moved a claimed PCI
//! function back to D0 for its holder, the broker puts back the BARs the
//! move lost (claim::restore_claimed, L62's readback). One that will not
//! stick loses the device: the claim is released as claim.rs releases it,
//! once, which stops its bus mastering and detaches its IOMMU domain.

use super::super::claim;
use super::super::pci_index;
use super::marf_rule::{check, Marf};
use crate::drivers::pci::capabilities::find_capability;
use crate::drivers::pci::config::ConfigSpace;

const PM_CAP: u8 = 0x01;
const PMCSR: u16 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RestoreError {
    Refused(Marf),
    /// A BAR would not take its value back; the claim is gone.
    Lost,
}

pub fn restore_function(device_id: u64) -> Result<(), RestoreError> {
    let handle = (device_id != 0).then(|| pci_index::lookup(device_id)).flatten();
    let holder = claim::lookup(device_id);
    let pm = handle.and_then(|h| {
        let a = h.address;
        let cap = find_capability(a.bus, a.device, a.function, PM_CAP)?;
        ConfigSpace::new(a).read16(u16::from(cap.offset) + PMCSR).ok()
    });
    check(device_id, handle.is_some(), holder.is_some(), pm).map_err(RestoreError::Refused)?;
    if claim::restore_claimed(device_id) {
        return Ok(());
    }
    if let Some(c) = holder {
        let _ = claim::release(c.pid, device_id);
    }
    Err(RestoreError::Lost)
}
