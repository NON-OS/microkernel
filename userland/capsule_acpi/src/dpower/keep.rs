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

//! The moves DEVICE_POWER refuses a PCI function because its configuration
//! would not survive them (`nonos_aml::device_power::refused`). A move is
//! made only for the function's claim holder (`target`, as
//! `Asker::Holder`). While the kernel offers MkAcpiRestoreFunction, which
//! puts a claimed function's BARs back once it is in D0 again, only D3cold
//! is refused; without it, D3hot needs No_Soft_Reset in the function's
//! power management status (capability id 1, register +4; recalled).
//! Decoding and bus mastering are the driver's to turn back on, as after
//! its claim.

use super::ready::{pci_function, Bdf};
use crate::boot::Acpi;
use crate::sys::acpi_pci::pci_read;
use crate::sys::pci_restore::{restore, restore_present};
use nonos_aml::device_power::{needs_restore, refused, DState};
use nonos_aml::namespace::Id;

const CAP_LIST: u16 = 0x34;
const PM_CAP: u32 = 0x01;
const PMCSR: u16 = 4;

fn pm_status(f: Bdf) -> Option<u16> {
    let mut at = pci_read(f, CAP_LIST, 1).ok()? as u16 & 0xFC;
    for _ in 0..48 {
        if !(0x40..=0xFC).contains(&at) {
            return None;
        }
        if pci_read(f, at, 1).ok()? == PM_CAP {
            return pci_read(f, at + PMCSR, 2).ok().map(|v| v as u16);
        }
        at = pci_read(f, at + 1, 1).ok()? as u16 & 0xFC;
    }
    None
}

/// Whether `dev` may move `from` -> `to` without losing what it holds.
pub fn keeps(acpi: &mut Acpi, dev: Id, from: DState, to: DState) -> bool {
    let Some(f) = pci_function(acpi, dev) else { return true };
    let restorable = restore_present();
    let pmcsr = if restorable { None } else { pm_status(f) };
    !refused(from, to, pmcsr, restorable)
}

/// After a move into D0 from D3: the kernel puts the BARs back before the
/// holder hears the move went through. The errno when it would not.
pub fn restore_after(
    acpi: &mut Acpi,
    dev: Id,
    id: u64,
    from: DState,
    to: DState,
) -> Result<(), i64> {
    let pci = needs_restore(from, to) && pci_function(acpi, dev).is_some();
    if pci && restore_present() {
        restore(id)?;
    }
    Ok(())
}
